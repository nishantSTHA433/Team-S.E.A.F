//! Deterministic, content-aware packing. Geometry is in logical pixels.
//! Pure candidate generation; the UI decides when to apply/animate a solution.
use crate::workspace::{AspectPreference, PanelId, Size};

const GUTTER: f64 = 8.0;
const CHROME: f64 = 48.0;
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}
impl Rect {
    pub fn area(self) -> f64 {
        self.width * self.height
    }
    pub fn overlaps(self, other: Self) -> bool {
        self.x < other.x + other.width - 0.001
            && self.x + self.width > other.x + 0.001
            && self.y < other.y + other.height - 0.001
            && self.y + self.height > other.y + 0.001
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct Placement {
    pub panel: PanelId,
    pub rect: Rect,
}
#[derive(Clone, Debug, PartialEq)]
pub struct Solution {
    pub panels: Vec<Placement>,
    pub emblem: Option<Rect>,
    pub canvas: Size,
    pub overflow: bool,
    topology: (usize, usize),
    score: f64,
    attention: Option<PanelId>,
}
impl Solution {
    pub fn panel(&self, id: PanelId) -> Option<Rect> {
        self.panels.iter().find(|p| p.panel == id).map(|p| p.rect)
    }
}
#[derive(Clone, Debug)]
pub struct Request {
    /// Available workspace after global chrome/keyboard occlusion.
    pub viewport: Size,
    pub panels: Vec<PanelId>,
    pub focused: Option<PanelId>,
    pub attention: Option<PanelId>,
}
#[derive(Clone, Copy)]
struct Item {
    id: Option<PanelId>,
    min_w: f64,
    min_h: f64,
    weight: f64,
}
fn lengths(minima: &[f64], weights: &[f64], total: f64) -> Vec<f64> {
    let mut result = minima.to_vec();
    let mut free = (total - minima.iter().sum::<f64>()).max(0.0);
    // Allocate toward weighted equality, protecting hard minima.
    let mut active = vec![true; minima.len()];
    loop {
        let sum = weights
            .iter()
            .zip(&active)
            .filter(|(_, a)| **a)
            .map(|(w, _)| *w)
            .sum::<f64>();
        if sum <= 0.0 {
            break;
        }
        let budget = free
            + minima
                .iter()
                .zip(&active)
                .filter(|(_, a)| **a)
                .map(|(m, _)| *m)
                .sum::<f64>();
        let mut removed = false;
        for i in 0..minima.len() {
            if active[i] && budget * weights[i] / sum < minima[i] - 0.001 {
                active[i] = false;
                removed = true;
            }
        }
        if !removed {
            for i in 0..result.len() {
                if active[i] {
                    result[i] = budget * weights[i] / sum;
                }
            }
            break;
        }
        // Free space stays unchanged when a minimum-sized item is frozen.
        free = free.max(0.0);
    }
    result
}
fn candidate(request: &Request, items: &[Item], insert: usize, mask: usize) -> Solution {
    let mut rows: Vec<Vec<Item>> = vec![vec![]];
    for (i, item) in items.iter().enumerate() {
        rows.last_mut().unwrap().push(*item);
        if i + 1 < items.len() && mask & (1 << i) != 0 {
            rows.push(vec![]);
        }
    }
    let row_min_h = rows
        .iter()
        .map(|r| r.iter().map(|i| i.min_h).fold(0.0, f64::max))
        .collect::<Vec<_>>();
    let row_weight = rows
        .iter()
        .map(|r| r.iter().map(|i| i.weight).sum())
        .collect::<Vec<f64>>();
    let min_w = rows
        .iter()
        .map(|r| r.iter().map(|i| i.min_w).sum::<f64>() + GUTTER * (r.len() - 1) as f64)
        .fold(0.0, f64::max);
    let min_h = row_min_h.iter().sum::<f64>() + GUTTER * (rows.len() - 1) as f64;
    let w = (request.viewport.width as f64).max(min_w);
    let h = (request.viewport.height as f64).max(min_h);
    let heights = lengths(
        &row_min_h,
        &row_weight,
        h - GUTTER * (rows.len() - 1) as f64,
    );
    let mut panels = Vec::new();
    let mut emblem = None;
    let mut y = 0.0;
    for (row, height) in rows.iter().zip(heights) {
        let widths = lengths(
            &row.iter().map(|i| i.min_w).collect::<Vec<_>>(),
            &row.iter().map(|i| i.weight).collect::<Vec<_>>(),
            w - GUTTER * (row.len() - 1) as f64,
        );
        let mut x = 0.0;
        for (item, width) in row.iter().zip(widths) {
            let rect = Rect {
                x,
                y,
                width,
                height,
            };
            if let Some(panel) = item.id {
                panels.push(Placement { panel, rect });
            } else {
                emblem = Some(rect);
            }
            x += width + GUTTER;
        }
        y += height + GUTTER;
    }
    panels.sort_by_key(|p| p.panel);
    let area = (w * h).max(1.0);
    let per_weight = panels
        .iter()
        .map(|p| {
            p.rect.area()
                / if request.attention == Some(p.panel) {
                    2.2
                } else {
                    1.0
                }
        })
        .collect::<Vec<_>>();
    let mean = per_weight.iter().sum::<f64>() / per_weight.len().max(1) as f64;
    let balance = per_weight
        .iter()
        .map(|a| ((a - mean) / mean.max(1.0)).powi(2))
        .sum::<f64>();
    let shape = panels
        .iter()
        .map(|p| {
            let ratio = p.rect.width / (p.rect.height - CHROME).max(1.0);
            let desired = match p.panel.contract().aspect {
                AspectPreference::Tall => 0.8,
                AspectPreference::Wide => 1.5,
                AspectPreference::Balanced => 1.0,
            };
            (ratio / desired).ln().powi(2)
        })
        .sum::<f64>();
    let preferred_deficit = panels
        .iter()
        .map(|p| {
            let c = p.panel.contract();
            ((c.preferred.width as f64 - p.rect.width).max(0.0) / c.preferred.width as f64).powi(2)
                + ((c.preferred.height as f64 + CHROME - p.rect.height).max(0.0)
                    / (c.preferred.height as f64 + CHROME))
                    .powi(2)
        })
        .sum::<f64>();
    let logo = emblem.map_or(0.0, |r| {
        let dx = (r.x + r.width / 2.0 - w / 2.0) / w.max(1.0);
        let dy = (r.y + r.height / 2.0 - h / 2.0) / h.max(1.0);
        (dx * dx + dy * dy) * 4.0 + (r.area() / area - 0.10).abs() * 3.0
    });
    let overflow_x = (w - request.viewport.width as f64) / request.viewport.width.max(1) as f64;
    let overflow_y = (h - request.viewport.height as f64) / request.viewport.height.max(1) as f64;
    let score = (overflow_x * 4.0 + overflow_y) * 10000.0
        + balance * 8.0
        + shape * 0.4
        + preferred_deficit * 0.5
        + logo;
    Solution {
        panels,
        emblem,
        canvas: Size {
            width: w.ceil() as u32,
            height: h.ceil() as u32,
        },
        overflow: overflow_x > 0.0 || overflow_y > 0.0,
        topology: (insert, mask),
        score,
        attention: request.attention,
    }
}
/// Enumerates row partitions (at most 448 candidates for six systems + emblem).
/// Retains the previous topology if within 8% of the best score and no greater
/// overflow, preventing small resizes from shuffling an otherwise useful layout.
pub fn solve(request: &Request, previous: Option<&Solution>) -> Solution {
    let ids = PanelId::ALL
        .into_iter()
        .filter(|id| request.panels.contains(id))
        .collect::<Vec<_>>();
    if let Some(id) = request.focused.filter(|id| ids.contains(id)) {
        let c = id.contract();
        let width = request.viewport.width.max(c.minimum.width);
        let height = request
            .viewport
            .height
            .max(c.minimum.height + CHROME as u32);
        return Solution {
            panels: vec![Placement {
                panel: id,
                rect: Rect {
                    x: 0.0,
                    y: 0.0,
                    width: width as f64,
                    height: height as f64,
                },
            }],
            emblem: None,
            canvas: Size { width, height },
            overflow: width > request.viewport.width || height > request.viewport.height,
            topology: (0, 0),
            score: 0.0,
            attention: None,
        };
    }
    if ids.is_empty() {
        return Solution {
            panels: vec![],
            emblem: Some(Rect {
                x: 0.0,
                y: 0.0,
                width: request.viewport.width as f64,
                height: request.viewport.height as f64,
            }),
            canvas: request.viewport,
            overflow: false,
            topology: (0, 0),
            score: 0.0,
            attention: None,
        };
    }
    let base = ids
        .iter()
        .map(|&id| {
            let c = id.contract();
            Item {
                id: Some(id),
                min_w: c.minimum.width as f64,
                min_h: (c.minimum.height as f64) + CHROME,
                weight: if request.attention == Some(id) {
                    2.2
                } else {
                    1.0
                },
            }
        })
        .collect::<Vec<_>>();
    let minimum_canvas_width = request
        .viewport
        .width
        .max(base.iter().map(|i| i.min_w as u32).max().unwrap_or(0));
    let mut best: Option<Solution> = None;
    let mut retained = None;
    for insert in 0..=base.len() {
        let mut items = base.clone();
        items.insert(
            insert,
            Item {
                id: None,
                min_w: 80.0,
                min_h: 80.0,
                weight: 0.6,
            },
        );
        for mask in 0..(1usize << (items.len() - 1)) {
            let result = candidate(request, &items, insert, mask);
            if result.canvas.width > minimum_canvas_width {
                continue;
            }
            if previous.is_some_and(|old| {
                old.topology == result.topology
                    && old.attention == result.attention
                    && old
                        .panels
                        .iter()
                        .map(|p| p.panel)
                        .eq(result.panels.iter().map(|p| p.panel))
            }) {
                retained = Some(result.clone());
            }
            if best
                .as_ref()
                .is_none_or(|old| result.score < old.score - 0.000001)
            {
                best = Some(result);
            }
        }
    }
    let best = best.unwrap();
    retained
        .filter(|old| old.canvas == best.canvas && old.score <= best.score * 1.08 + 0.08)
        .unwrap_or(best)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn req(width: u32, height: u32) -> Request {
        Request {
            viewport: (width, height).into(),
            panels: PanelId::ALL.to_vec(),
            focused: None,
            attention: None,
        }
    }
    fn check(s: &Solution) {
        let mut rects = s.panels.iter().map(|p| p.rect).collect::<Vec<_>>();
        if let Some(r) = s.emblem {
            rects.push(r);
        }
        for (i, r) in rects.iter().enumerate() {
            assert!(r.x >= 0.0 && r.y >= 0.0 && r.width >= 0.0 && r.height >= 0.0);
            assert!(
                r.x + r.width <= s.canvas.width as f64 + 0.001
                    && r.y + r.height <= s.canvas.height as f64 + 0.001
            );
            for other in &rects[i + 1..] {
                assert!(!r.overlaps(*other));
            }
        }
        for p in &s.panels {
            let c = p.panel.contract();
            assert!(p.rect.width + 0.001 >= c.minimum.width as f64);
            assert!(p.rect.height + 0.001 >= c.minimum.height as f64 + CHROME);
        }
    }
    #[test]
    fn all_subsets_and_aspect_ratios_preserve_bounds_and_readable_minima() {
        for mask in 0..64 {
            for (w, h) in [
                (0, 0),
                (320, 480),
                (800, 480),
                (1000, 600),
                (1280, 800),
                (1920, 1080),
                (600, 1400),
                (2400, 400),
            ] {
                let mut r = req(w, h);
                r.panels.retain(|id| mask & (1 << id.index()) != 0);
                let s = solve(&r, None);
                check(&s);
                assert_eq!(s.panels.len(), r.panels.len());
                assert_eq!(s, solve(&r, None));
            }
        }
    }
    #[test]
    fn focus_uses_whole_viewport_and_hides_other_panels() {
        let mut r = req(1000, 700);
        r.focused = Some(PanelId::Files);
        let s = solve(&r, None);
        assert_eq!(s.panels.len(), 1);
        assert_eq!(s.emblem, None);
        assert_eq!(s.panel(PanelId::Files).unwrap().area(), 700000.0);
        check(&s);
    }
    #[test]
    fn small_viewport_reports_overflow_instead_of_shrinking_text() {
        let s = solve(&req(320, 480), None);
        assert!(s.overflow);
        assert_eq!(s.canvas.width, 320);
        check(&s);
    }
    #[test]
    fn dense_desktop_and_attention_remain_useful() {
        let mut r = req(1600, 1000);
        let idle = solve(&r, None);
        assert!(!idle.overflow);
        let area =
            idle.panels.iter().map(|p| p.rect.area()).sum::<f64>() + idle.emblem.unwrap().area();
        assert!(area / (1600.0 * 1000.0) > 0.95);
        let min = idle
            .panels
            .iter()
            .map(|p| p.rect.area())
            .fold(f64::INFINITY, f64::min);
        let max = idle
            .panels
            .iter()
            .map(|p| p.rect.area())
            .fold(0.0, f64::max);
        assert!(max / min < 1.2);
        r.attention = Some(PanelId::Files);
        let active = solve(&r, Some(&idle));
        check(&active);
        assert!(
            active.panel(PanelId::Files).unwrap().area()
                > idle.panel(PanelId::Files).unwrap().area() * 1.4
        );
    }
    #[test]
    fn tiny_resize_keeps_topology_and_duplicates_do_not_duplicate_panels() {
        let mut r = req(1600, 1000);
        let s = solve(&r, None);
        r.viewport.width += 1;
        let next = solve(&r, Some(&s));
        assert_eq!(next.topology, s.topology);
        r.panels.push(PanelId::Files);
        assert_eq!(solve(&r, None).panels.len(), 6);
    }
}
