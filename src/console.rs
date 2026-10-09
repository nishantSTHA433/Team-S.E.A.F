use super::{button, column, draw_emblem, label, row, Navigate, SetMode, PANELS};
use crate::services::{self, Preferences};
use gtk::{glib, prelude::*};
use serde::{Deserialize, Serialize};
use std::cell::{Cell, RefCell};
use std::rc::Rc;

#[path = "overview.rs"]
mod overview;

#[derive(Default)]
pub(super) struct InstrumentData {
    pub(super) sources: overview::Sources,
    pub(super) demand: super::TaskDemand,
    snapshot: Option<services::Snapshot>,
    history: std::collections::VecDeque<f64>,
    traffic: std::collections::VecDeque<(f64, f64)>,
    updated: Option<std::time::Instant>,
}
impl InstrumentData {
    pub(super) fn update(&mut self, snapshot: &services::Snapshot) {
        self.snapshot = Some(snapshot.clone());
        self.updated = Some(std::time::Instant::now());
        if self.history.len() == 60 {
            self.history.pop_front();
        }
        self.history.push_back(snapshot.cpu.clamp(0.0, 100.0));
        if self.traffic.len() == 60 {
            self.traffic.pop_front();
        }
        self.traffic.push_back((
            snapshot.received as f64 / 1024.0,
            snapshot.transmitted as f64 / 1024.0,
        ));
    }
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
struct Region {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
}
impl Region {
    fn bounded(self) -> Self {
        let clean = |v: f64, fallback: f64| if v.is_finite() { v } else { fallback };
        let w = clean(self.w, 0.3).clamp(0.22, 1.0);
        let h = clean(self.h, 0.43).clamp(0.25, 1.0);
        Self {
            x: clean(self.x, 0.0).clamp(0.0, 1.0 - w),
            y: clean(self.y, 0.0).clamp(0.0, 1.0 - h),
            w,
            h,
        }
    }
    fn snap(self) -> Self {
        let snap = |v: f64| (v * 40.0).round() / 40.0;
        Self {
            x: snap(self.x),
            y: snap(self.y),
            w: snap(self.w),
            h: snap(self.h),
        }
        .bounded()
    }
}
// Equal-sized slots reserve the center for the ship emblem.
fn automatic(visible: &[usize]) -> Vec<Region> {
    let mut result = vec![
        Region {
            x: 0.0,
            y: 0.0,
            w: 0.31,
            h: 0.36
        };
        PANELS.len()
    ];
    let rect = |x, y, w, h| Region { x, y, w, h };
    let slots = match visible.len() {
        0 => vec![],
        1 => vec![rect(0.0, 0.0, 0.64, 1.0)],
        2 => vec![rect(0.0, 0.0, 0.39, 1.0), rect(0.61, 0.0, 0.39, 1.0)],
        3 => vec![
            rect(0.0, 0.0, 0.286, 1.0),
            rect(0.714, 0.0, 0.286, 1.0),
            rect(0.300, 0.0, 0.4, 0.492),
        ],
        4 => vec![
            rect(0.0, 0.0, 0.33, 0.492),
            rect(0.67, 0.0, 0.33, 0.492),
            rect(0.67, 0.508, 0.33, 0.492),
            rect(0.0, 0.508, 0.33, 0.492),
        ],
        5 => vec![
            rect(0.0, 0.0, 0.286, 0.492),
            rect(0.714, 0.0, 0.286, 0.492),
            rect(0.714, 0.508, 0.286, 0.492),
            rect(0.0, 0.508, 0.286, 0.492),
            rect(0.300, 0.0, 0.4, 0.492),
        ],
        _ => vec![
            rect(0.0, 0.0, 0.286, 0.492),
            rect(0.714, 0.0, 0.286, 0.492),
            rect(0.714, 0.508, 0.286, 0.492),
            rect(0.0, 0.508, 0.286, 0.492),
            rect(0.300, 0.0, 0.4, 0.35),
            rect(0.300, 0.65, 0.4, 0.35),
        ],
    };
    for (&index, region) in visible.iter().zip(slots) {
        result[index] = region;
    }
    result
}
fn defaults() -> Vec<Region> {
    automatic(&(0..PANELS.len()).collect::<Vec<_>>())
}
fn interpolate(a: Region, b: Region, t: f64) -> Region {
    let mix = |a: f64, b: f64| a + (b - a) * t;
    Region {
        x: mix(a.x, b.x),
        y: mix(a.y, b.y),
        w: mix(a.w, b.w),
        h: mix(a.h, b.h),
    }
}
fn drop_target(
    regions: &[Region],
    hidden: &[bool],
    source: usize,
    x: f64,
    y: f64,
) -> Option<usize> {
    regions.iter().enumerate().rev().find_map(|(i, r)| {
        (i != source && !hidden[i] && x >= r.x && x <= r.x + r.w && y >= r.y && y <= r.y + r.h)
            .then_some(i)
    })
}
fn logo_region(regions: &[Region]) -> Region {
    let mut xs = vec![0.0, 1.0];
    let mut ys = vec![0.0, 1.0];
    for r in regions {
        xs.extend([r.x, r.x + r.w]);
        ys.extend([r.y, r.y + r.h]);
    }
    xs.sort_by(f64::total_cmp);
    xs.dedup();
    ys.sort_by(f64::total_cmp);
    ys.dedup();
    let mut best = Region {
        x: 0.5,
        y: 0.5,
        w: 0.0,
        h: 0.0,
    };
    let mut score = 0.0;
    for (a, &x) in xs.iter().enumerate() {
        for &right in &xs[a + 1..] {
            for (b, &y) in ys.iter().enumerate() {
                for &bottom in &ys[b + 1..] {
                    if regions.iter().any(|r| {
                        x < r.x + r.w - 0.00001
                            && right > r.x + 0.00001
                            && y < r.y + r.h - 0.00001
                            && bottom > r.y + 0.00001
                    }) {
                        continue;
                    }
                    let w = right - x;
                    let h = bottom - y;
                    let candidate = w * h
                        / (1.0 + ((x + w / 2.0 - 0.5).powi(2) + (y + h / 2.0 - 0.5).powi(2)) * 2.0);
                    if candidate > score {
                        score = candidate;
                        best = Region { x, y, w, h };
                    }
                }
            }
        }
    }
    best
}
fn layout_path() -> std::path::PathBuf {
    std::env::var_os("XDG_CONFIG_HOME")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| services::home().join(".config"))
        .join("lcars-desktop/console-layout-v2.json")
}
#[derive(Serialize, Deserialize)]
struct SavedLayout {
    regions: Vec<Region>,
    hidden: Vec<bool>,
    #[serde(default)]
    automatic: bool,
}
fn load() -> SavedLayout {
    std::fs::read(layout_path())
        .ok()
        .and_then(|b| serde_json::from_slice::<SavedLayout>(&b).ok())
        .filter(|v| v.regions.len() == PANELS.len() && v.hidden.len() == PANELS.len())
        .map(|v| SavedLayout {
            regions: v.regions.into_iter().map(Region::bounded).collect(),
            hidden: v.hidden,
            automatic: v.automatic,
        })
        .unwrap_or_else(|| SavedLayout {
            regions: defaults(),
            hidden: vec![false; PANELS.len()],
            automatic: true,
        })
}
fn save(regions: &[Region], hidden: &[bool], automatic: bool) -> Result<(), String> {
    let path = layout_path();
    std::fs::create_dir_all(path.parent().ok_or("Missing configuration directory")?)
        .map_err(|e| e.to_string())?;
    let bytes = serde_json::to_vec_pretty(&SavedLayout {
        regions: regions.to_vec(),
        hidden: hidden.to_vec(),
        automatic,
    })
    .map_err(|e| e.to_string())?;
    let temp = path.with_extension("tmp");
    std::fs::write(&temp, bytes).map_err(|e| e.to_string())?;
    std::fs::rename(temp, path).map_err(|e| e.to_string())
}
struct Surface {
    demand: super::TaskDemand,
    attention: Cell<Option<lcars_desktop::workspace::PanelId>>,
    attention_base: RefCell<Option<(Vec<Region>, Option<lcars_desktop::layout::Solution>)>>,
    pressing: Cell<bool>,
    shell: gtk::Overlay,
    viewport: gtk::ScrolledWindow,
    auto_layout: Cell<bool>,
    solution: RefCell<Option<lcars_desktop::layout::Solution>>,
    canvas: gtk::Fixed,
    background: gtk::DrawingArea,
    regions: RefCell<Vec<Region>>,
    tiles: Vec<gtk::Overlay>,
    interiors: Vec<gtk::Fixed>,
    pages: Vec<gtk::Widget>,
    headings: Vec<gtk::Button>,
    titles: Vec<gtk::Label>,
    covers: Vec<gtk::Button>,
    handles: Vec<gtk::Button>,
    focused: Cell<Option<usize>>,
    input_memory: RefCell<Vec<Option<glib::WeakRef<gtk::Widget>>>>,
    restored_input: Cell<Option<usize>>,
    selected: Cell<usize>,
    arranging: Cell<bool>,
    active: Cell<bool>,
    dimensions: Cell<(i32, i32)>,
    status: gtk::Label,
    overviews: Vec<overview::Overview>,
    maximizers: Vec<gtk::Button>,
    minimizers: Vec<gtk::Button>,
    advanced: Cell<bool>,
    hidden: RefCell<Vec<bool>>,
    shown: RefCell<Vec<Region>>,
    transition: RefCell<Option<(std::time::Instant, Vec<Region>)>>,
    mode_picker: gtk::DropDown,
    edits: gtk::Box,
    arrange_button: gtk::Button,
    command: gtk::Box,
    swap_source: Cell<Option<usize>>,
    drop_hover: Cell<Option<usize>>,
}
impl Surface {
    fn update_attention(&self) {
        if !self.active.get()
            || self.focused.get().is_some()
            || self.arranging.get()
            || self.pressing.get()
        {
            return;
        }
        if self
            .shell
            .root()
            .and_then(|r| gtk::prelude::RootExt::focus(&r))
            .is_some_and(|w| w.is::<gtk::Text>())
        {
            return;
        }
        let next = if self.auto_layout.get() {
            self.demand
                .borrow()
                .preferred(|p| !self.hidden.borrow()[p.index()])
        } else {
            None
        };
        if next == self.attention.get() {
            return;
        }
        if self.attention.get().is_none() && next.is_some() {
            *self.attention_base.borrow_mut() = Some((
                self.regions.borrow().clone(),
                self.solution.borrow().clone(),
            ));
        }
        self.attention.set(next);
        if next.is_none() {
            if let Some((regions, solution)) = self.attention_base.borrow_mut().take() {
                *self.regions.borrow_mut() = regions;
                *self.solution.borrow_mut() = solution;
            }
        }
        self.pack();
        self.animate();
    }

    fn remember_input(&self) {
        if self.active.get() && self.focused.get() != self.restored_input.get() {
            return;
        }
        let Some(root) = self.shell.root() else {
            return;
        };
        let Some(input) = gtk::prelude::RootExt::focus(&root) else {
            return;
        };
        if !input.is::<gtk::Text>() && !input.is::<gtk::Entry>() && !input.is::<gtk::SearchEntry>()
        {
            return;
        }
        for (i, page) in self.pages.iter().enumerate() {
            if input.is_ancestor(page) && input.is_sensitive() && input.is_mapped() {
                self.input_memory.borrow_mut()[i] = Some(input.downgrade());
                break;
            }
        }
    }

    fn suspend_input(&self) {
        self.remember_input();
        if let Some(root) = self.shell.root() {
            if gtk::prelude::RootExt::focus(&root)
                .is_some_and(|input| self.pages.iter().any(|page| input.is_ancestor(page)))
            {
                gtk::prelude::RootExt::set_focus(&root, None::<&gtk::Widget>);
            }
        }
    }
    fn persist(&self) {
        self.status.set_text(&match save(
            &self
                .attention_base
                .borrow()
                .as_ref()
                .map(|(r, _)| r.clone())
                .unwrap_or_else(|| self.regions.borrow().clone()),
            &self.hidden.borrow(),
            self.auto_layout.get(),
        ) {
            Ok(()) => "ARRANGEMENT SAVED".into(),
            Err(e) => format!("SAVE FAILED / {e}"),
        });
    }
    fn toggle_arrange(&self) {
        self.suspend_input();
        self.restored_input.set(None);
        let on = !self.arranging.get();
        self.arranging.set(on);
        if on {
            self.attention.set(None);
            if let Some((regions, solution)) = self.attention_base.borrow_mut().take() {
                *self.regions.borrow_mut() = regions;
                *self.solution.borrow_mut() = solution;
            }
            self.auto_layout.set(false);
        }
        self.focused.set(None);
        self.advanced.set(false);
        self.mode_picker.set_selected(0);
        self.swap_source.set(None);
        self.sync_controls();
        self.status.set_text(if on {
            "BASIC / DRAG ONTO ANOTHER SYSTEM TO SWAP"
        } else {
            "HOME / TOUCH A SYSTEM TO ZOOM IN"
        });
        if !on {
            self.persist();
        }
        self.animate();
    }
    fn home(&self) {
        self.suspend_input();
        self.restored_input.set(None);
        self.focused.set(None);
        if self.auto_layout.get() {
            self.pack();
        }
        self.arranging.set(false);
        self.sync_controls();
        self.status.set_text("HOME / TOUCH A SYSTEM TO ZOOM IN");
        self.animate();
    }
    fn sync_controls(&self) {
        let arranging = self.arranging.get();
        self.command.set_visible(arranging);
        self.mode_picker.set_visible(arranging);
        self.edits.set_visible(arranging && self.advanced.get());
        self.arrange_button
            .set_label(if arranging { "DONE" } else { "ARRANGE" });
    }
    fn animate(&self) {
        *self.transition.borrow_mut() =
            Some((std::time::Instant::now(), self.shown.borrow().clone()));
        self.layout();
    }
    fn reflow(&self) {
        self.auto_layout.set(true);
        self.pack();
    }
    fn pack(&self) {
        use lcars_desktop::{
            layout::{self, Request},
            workspace::PanelId,
        };
        let request = Request {
            viewport: (
                (self.viewport.width() - 6).max(1) as u32,
                (self.viewport.height() - 6).max(1) as u32,
            )
                .into(),
            panels: PanelId::ALL
                .into_iter()
                .filter(|id| !self.hidden.borrow()[id.index()])
                .collect(),
            focused: None,
            attention: self.attention.get(),
        };
        let result = layout::solve(&request, self.solution.borrow().as_ref());
        let width = (result.canvas.width + 6).max(1) as f64;
        let height = (result.canvas.height + 6).max(1) as f64;
        for p in &result.panels {
            self.regions.borrow_mut()[p.panel.index()] = Region {
                x: p.rect.x / width,
                y: p.rect.y / height,
                w: (p.rect.width + 6.0) / width,
                h: (p.rect.height + 6.0) / height,
            };
        }
        *self.solution.borrow_mut() = Some(result);
    }
    fn maximize(&self, index: usize) {
        self.suspend_input();
        self.restored_input.set(None);
        self.hidden.borrow_mut()[index] = false;
        self.arranging.set(false);
        self.sync_controls();
        self.focused.set(Some(index));
        self.status
            .set_text("FOCUSED / TOUCH THE TITLE AGAIN TO RETURN HOME");
        self.animate();
    }
    fn minimize(&self, index: usize) {
        self.suspend_input();
        self.restored_input.set(None);
        self.hidden.borrow_mut()[index] = true;
        self.focused.set(None);
        self.swap_source.set(None);
        self.reflow();
        self.persist();
        self.status
            .set_text("MINIMIZED / REOPEN FROM START OR THE TASKBAR");
        self.animate();
    }
    fn layout(&self) {
        self.remember_input();
        let dimensions = (self.viewport.width(), self.viewport.height());
        let focus = self.focused.get();
        if self.auto_layout.get()
            && focus.is_none()
            && !self.arranging.get()
            && (self.dimensions.get() != dimensions || self.solution.borrow().is_none())
        {
            self.pack();
        }
        self.dimensions.set(dimensions);
        let canvas_size = if focus.is_none() && self.auto_layout.get() {
            self.solution
                .borrow()
                .as_ref()
                .map(|s| ((s.canvas.width + 6) as i32, (s.canvas.height + 6) as i32))
                .unwrap_or(dimensions)
        } else {
            dimensions
        };
        let width = canvas_size.0.max(1) as f64;
        let height = canvas_size.1.max(1) as f64;
        self.shell
            .set_size_request(canvas_size.0.max(1), canvas_size.1.max(1));
        let transition = self.transition.borrow().clone();
        let progress = transition.as_ref().map_or(1.0, |(start, _)| {
            (start.elapsed().as_secs_f64() / 0.30).min(1.0)
        });
        let ease = progress * progress * (3.0 - 2.0 * progress);
        for (i, (_, title)) in PANELS.iter().enumerate() {
            let visible = !self.hidden.borrow()[i] && focus.is_none_or(|f| i == f);
            self.tiles[i].set_visible(visible);
            self.tiles[i].set_can_target(visible);
            if !visible {
                continue;
            }
            let target = if focus == Some(i) {
                Region {
                    x: 0.0,
                    y: 0.0,
                    w: 1.0,
                    h: 1.0,
                }
            } else {
                self.regions.borrow()[i]
            };
            let r = transition
                .as_ref()
                .map_or(target, |(_, from)| interpolate(from[i], target, ease));
            self.shown.borrow_mut()[i] = r;
            let w = (r.w * width - 6.0).max(100.0) as i32;
            let h = (r.h * height - 6.0).max(80.0) as i32;
            self.tiles[i].set_size_request(w, h);
            self.canvas.move_(&self.tiles[i], r.x * width, r.y * height);
            let content_h = (h - 52).max(1);
            let raw_w = if focus.is_some() {
                (width - 6.0).max(1.0) as i32
            } else {
                960
            };
            let raw_h = if focus.is_some() {
                (height - 58.0).max(1.0) as i32
            } else {
                ((960.0 * content_h as f64 / w as f64) as i32).max(1)
            };
            self.pages[i].set_visible(focus.is_some());
            self.overviews[i].widget.set_visible(focus.is_none());
            self.overviews[i].widget.set_size_request(w, content_h);
            self.pages[i].set_size_request(raw_w, raw_h);
            let interactive = focus == Some(i) && progress >= 1.0;
            self.pages[i].set_can_target(interactive);
            self.pages[i].set_sensitive(interactive);
            self.pages[i].set_focusable(interactive);
            let scale = (w as f32 / raw_w as f32).min(content_h as f32 / raw_h as f32);
            self.interiors[i].set_child_transform(
                &self.pages[i],
                Some(&gtk::gsk::Transform::new().scale(scale, scale)),
            );
            self.covers[i].set_visible(focus.is_none());
            self.handles[i].set_visible(self.arranging.get() && self.advanced.get());
            self.maximizers[i].set_sensitive(!self.arranging.get());
            self.minimizers[i].set_sensitive(!self.arranging.get());
            self.titles[i].set_text(&format!("{:02} / {}", i + 1, title));
            if (self.arranging.get() && self.selected.get() == i)
                || self.drop_hover.get() == Some(i)
            {
                self.tiles[i].add_css_class("arrange-selected");
            } else {
                self.tiles[i].remove_css_class("arrange-selected");
            }
        }
        if progress >= 1.0 {
            self.transition.borrow_mut().take();
            if let Some(i) = focus {
                if self.restored_input.get() != Some(i) && self.pages[i].is_mapped() {
                    if let Some(input) = self.input_memory.borrow()[i]
                        .as_ref()
                        .and_then(|w| w.upgrade())
                    {
                        if input.is_ancestor(&self.pages[i])
                            && input.is_sensitive()
                            && input.is_visible()
                        {
                            input.grab_focus();
                        }
                    }
                    self.restored_input.set(Some(i));
                }
            }
        }
        self.background.queue_draw();
    }
    fn choose(&self, index: usize) {
        self.selected.set(index);
        if self.arranging.get() {
            if !self.advanced.get() {
                if let Some(from) = self.swap_source.take() {
                    if from != index {
                        self.regions.borrow_mut().swap(from, index);
                        self.persist();
                        self.animate();
                    }
                } else {
                    self.swap_source.set(Some(index));
                    self.status
                        .set_text("TOUCH ANOTHER SYSTEM TO SWAP / OR DRAG AND DROP");
                }
            }
            self.layout();
        } else if self.focused.get() == Some(index) {
            self.home();
        } else {
            if self.hidden.borrow()[index] {
                self.hidden.borrow_mut()[index] = false;
                self.reflow();
                self.persist();
            }
            self.maximize(index);
        }
    }
}

pub(super) fn build(
    single: &gtk::Stack,
    prefs: &Rc<RefCell<Preferences>>,
    data: Rc<RefCell<InstrumentData>>,
) -> (gtk::Stack, Navigate, SetMode) {
    let host = gtk::Stack::new();
    host.set_hexpand(true);
    host.set_vexpand(true);
    host.add_named(single, Some("single"));
    let console = column(6);
    console.add_css_class("console-surface");
    let command = row(6);
    command.set_visible(false);
    let overview = button("ALL SYSTEMS", "primary");
    let arrange = button("ARRANGE", "secondary");
    let auto = button("AUTO FIT", "secondary");
    command.append(&overview);
    command.append(&arrange);
    command.append(&auto);
    let mode_picker = gtk::DropDown::from_strings(&["BASIC", "ADVANCED"]);
    mode_picker.set_height_request(48);
    mode_picker.add_css_class("arrange-mode");
    mode_picker.set_visible(false);
    command.append(&mode_picker);
    let status = label("TAP A SYSTEM TO ZOOM IN", "muted");
    status.set_hexpand(true);
    command.append(&status);
    console.append(&command);
    let edits = row(6);
    edits.set_visible(false);
    for text in ["←", "→", "↑", "↓", "SMALLER", "LARGER"] {
        edits.append(&button(text, "secondary"));
    }
    console.append(&edits);
    let shell = gtk::Overlay::new();
    shell.set_hexpand(true);
    shell.set_vexpand(true);
    shell.set_overflow(gtk::Overflow::Hidden);
    let background = gtk::DrawingArea::new();
    background.set_hexpand(true);
    background.set_vexpand(true);
    shell.set_child(Some(&background));
    let canvas = gtk::Fixed::new();
    canvas.set_hexpand(true);
    canvas.set_vexpand(true);
    shell.add_overlay(&canvas);
    shell.set_measure_overlay(&canvas, false);
    let viewport = gtk::ScrolledWindow::builder()
        .child(&shell)
        .hexpand(true)
        .vexpand(true)
        .hscrollbar_policy(gtk::PolicyType::Automatic)
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .build();
    console.append(&viewport);
    host.add_named(&console, Some("workspace"));
    let pages: Vec<gtk::Widget> = PANELS
        .iter()
        .map(|(name, _)| single.child_by_name(name).expect("Registered system"))
        .collect();
    let mut tiles = Vec::new();
    let mut interiors = Vec::new();
    let mut headings = Vec::new();
    let mut titles = Vec::new();
    let mut covers = Vec::new();
    let mut handles = Vec::new();
    let mut maximizers = Vec::new();
    let mut minimizers = Vec::new();
    for (route, title) in PANELS {
        let tile = gtk::Overlay::new();
        tile.set_widget_name(&format!("lcars-panel-{route}"));
        tile.add_css_class("console-region");
        tile.set_overflow(gtk::Overflow::Hidden);
        let bounds = gtk::DrawingArea::new();
        tile.set_child(Some(&bounds));
        let body = column(4);
        let heading = button("", "system-band");
        heading.set_hexpand(true);
        heading.set_height_request(48);
        let title_text = label(title, "console-band-title");
        title_text.set_valign(gtk::Align::Center);
        title_text.set_wrap(false);
        title_text.set_ellipsize(gtk::pango::EllipsizeMode::End);
        heading.set_child(Some(&title_text));
        let band = row(3);
        band.append(&heading);
        for (icon, class, tip, collection) in [
            (
                "□",
                "window-maximize",
                "Maximize this system",
                &mut maximizers,
            ),
            (
                "−",
                "window-minimize",
                "Minimize / reopen from Start",
                &mut minimizers,
            ),
        ] {
            let control = button(icon, class);
            control.set_size_request(44, 48);
            let glyph = label(icon, "window-glyph");
            glyph.set_valign(gtk::Align::Center);
            glyph.set_justify(gtk::Justification::Center);
            glyph.set_xalign(0.5);
            control.set_child(Some(&glyph));
            control.set_tooltip_text(Some(tip));
            band.append(&control);
            collection.push(control);
        }
        titles.push(title_text);
        body.append(&band);
        let interior = gtk::Fixed::new();
        interior.set_hexpand(true);
        interior.set_vexpand(true);
        interior.set_overflow(gtk::Overflow::Hidden);
        body.append(&interior);
        tile.add_overlay(&body);
        tile.set_measure_overlay(&body, false);
        let cover = gtk::Button::new();
        cover.add_css_class("zoom-target");
        cover.set_margin_top(52);
        cover.set_tooltip_text(Some("Tap to focus this system"));
        tile.add_overlay(&cover);
        let handle = button("↘", "resize-handle");
        handle.set_width_request(52);
        handle.set_halign(gtk::Align::End);
        handle.set_valign(gtk::Align::End);
        handle.set_visible(false);
        tile.add_overlay(&handle);
        canvas.put(&tile, 0.0, 0.0);
        tiles.push(tile);
        interiors.push(interior);
        headings.push(heading);
        covers.push(cover);
        handles.push(handle);
    }
    let overviews = (0..PANELS.len())
        .map(|i| {
            let overview = overview::build(i, data.clone(), prefs.clone(), &covers[i]);
            overview
                .widget
                .update_property(&[gtk::accessible::Property::Label(PANELS[i].1)]);
            interiors[i].put(&overview.widget, 0.0, 0.0);
            overview
        })
        .collect();
    let saved = load();
    let surface = Rc::new(Surface {
        demand: data.borrow().demand.clone(),
        attention: Cell::new(None),
        attention_base: RefCell::new(None),
        pressing: Cell::new(false),
        shell,
        viewport,
        auto_layout: Cell::new(saved.automatic),
        solution: RefCell::new(None),
        canvas,
        background,
        regions: RefCell::new(saved.regions.clone()),
        shown: RefCell::new(saved.regions),
        tiles,
        interiors,
        pages,
        headings,
        titles,
        covers,
        handles,
        focused: Cell::new(None),
        input_memory: RefCell::new(vec![None; PANELS.len()]),
        restored_input: Cell::new(None),
        selected: Cell::new(0),
        arranging: Cell::new(false),
        active: Cell::new(false),
        dimensions: Cell::new((0, 0)),
        status,
        overviews,
        maximizers,
        minimizers,
        advanced: Cell::new(false),
        hidden: RefCell::new(saved.hidden),
        transition: RefCell::new(None),
        mode_picker: mode_picker.clone(),
        edits: edits.clone(),
        arrange_button: arrange.clone(),
        command: command.clone(),
        swap_source: Cell::new(None),
        drop_hover: Cell::new(None),
    });
    let press = gtk::GestureClick::new();
    press.set_propagation_phase(gtk::PropagationPhase::Capture);
    let weak = Rc::downgrade(&surface);
    press.connect_pressed(move |_, _, _, _| {
        if let Some(s) = weak.upgrade() {
            s.pressing.set(true);
        }
    });
    let weak = Rc::downgrade(&surface);
    press.connect_released(move |_, _, _, _| {
        if let Some(s) = weak.upgrade() {
            s.pressing.set(false);
        }
    });
    let weak = Rc::downgrade(&surface);
    press.connect_cancel(move |_, _| {
        if let Some(s) = weak.upgrade() {
            s.pressing.set(false);
        }
    });
    host.add_controller(press);
    let weak = Rc::downgrade(&surface);
    glib::timeout_add_local(std::time::Duration::from_millis(200), move || {
        let Some(s) = weak.upgrade() else {
            return glib::ControlFlow::Break;
        };
        s.update_attention();
        glib::ControlFlow::Continue
    });
    let weak = Rc::downgrade(&surface);
    surface.background.set_draw_func(move |area, cr, w, h| {
        let Some(s) = weak.upgrade() else { return };
        if s.focused.get().is_some() {
            return;
        }
        let visible = s
            .shown
            .borrow()
            .iter()
            .enumerate()
            .filter(|(i, _)| !s.hidden.borrow()[*i])
            .map(|(_, r)| *r)
            .collect::<Vec<_>>();
        let gap = logo_region(&visible);
        let x = gap.x * w as f64 + 5.0;
        let y = gap.y * h as f64 + 5.0;
        let width = (gap.w * w as f64 - 10.0).max(1.0);
        let height = (gap.h * h as f64 - 10.0).max(1.0);
        let horizontal = width / height > 2.2;
        let (design_w, design_h) = if horizontal {
            (480.0, 160.0)
        } else {
            (210.0, 215.0)
        };
        let scale = (width / design_w).min(height / design_h).min(2.8);
        if scale < 0.15 {
            return;
        }
        let _ = cr.save();
        cr.translate(
            x + (width - design_w * scale) / 2.0,
            y + (height - design_h * scale) / 2.0,
        );
        cr.scale(scale, scale);
        let _ = cr.save();
        cr.translate(if horizontal { 0.0 } else { 25.0 }, 0.0);
        cr.translate(80.0, 80.0);
        cr.scale(1.22, 1.22);
        cr.translate(-80.0, -80.0);
        draw_emblem(area, cr, 160, 160);
        let _ = cr.restore();
        cr.select_font_face(
            "Nimbus Sans Narrow",
            gtk::cairo::FontSlant::Normal,
            gtk::cairo::FontWeight::Bold,
        );
        for (text, baseline, font_size, color) in [
            (
                "SYSTEM AVAILABLE",
                if horizontal { 80.0 } else { 188.0 },
                if horizontal { 34.0 } else { 27.0 },
                (0.70, 0.64, 0.89),
            ),
            (
                "STATION ID 1440–900",
                if horizontal { 107.0 } else { 211.0 },
                if horizontal { 22.0 } else { 17.0 },
                (0.80, 0.45, 0.49),
            ),
        ] {
            cr.set_font_size(font_size);
            cr.set_source_rgb(color.0, color.1, color.2);
            if let Ok(extent) = cr.text_extents(text) {
                let center = if horizontal { 325.0 } else { 105.0 };
                cr.move_to(center - extent.width() / 2.0 - extent.x_bearing(), baseline);
                let _ = cr.show_text(text);
            }
        }
        let _ = cr.restore();
    });
    for index in 0..PANELS.len() {
        let scroll = gtk::EventControllerScroll::new(gtk::EventControllerScrollFlags::VERTICAL);
        let weak = Rc::downgrade(&surface);
        scroll.connect_scroll(move |_, _, dy| {
            if let Some(s) = weak.upgrade() {
                if !s.arranging.get() {
                    let view = &s.overviews[index];
                    if index == 1 || index == 3 {
                        let offset = view.offset.get();
                        view.offset.set(if dy > 0.0 {
                            offset.saturating_add(1)
                        } else {
                            offset.saturating_sub(1)
                        });
                        view.canvas.queue_draw();
                    } else {
                        let adjustment = view.widget.vadjustment();
                        adjustment.set_value((adjustment.value() + dy * 32.0).clamp(
                            adjustment.lower(),
                            (adjustment.upper() - adjustment.page_size()).max(adjustment.lower()),
                        ));
                    }
                    return glib::Propagation::Stop;
                }
            }
            glib::Propagation::Proceed
        });
        surface.covers[index].add_controller(scroll);
        let swipe = gtk::GestureDrag::new();
        swipe.set_propagation_phase(gtk::PropagationPhase::Capture);
        let origin = Rc::new(Cell::new((0usize, 0.0)));
        let start = origin.clone();
        let weak = Rc::downgrade(&surface);
        swipe.connect_drag_begin(move |_, _, _| {
            if let Some(s) = weak.upgrade() {
                start.set((
                    s.overviews[index].offset.get(),
                    s.overviews[index].widget.vadjustment().value(),
                ));
            }
        });
        let weak = Rc::downgrade(&surface);
        swipe.connect_drag_update(move |gesture, _, dy| {
            if let Some(s) = weak.upgrade() {
                if !s.arranging.get() && s.focused.get().is_none() && dy.abs() > 10.0 {
                    gesture.set_state(gtk::EventSequenceState::Claimed);
                    let view = &s.overviews[index];
                    let (row, value) = origin.get();
                    if index == 1 || index == 3 {
                        view.offset.set((row as f64 - dy / 28.0).max(0.0) as usize);
                        view.canvas.queue_draw();
                    } else {
                        let a = view.widget.vadjustment();
                        a.set_value(
                            (value - dy)
                                .clamp(a.lower(), (a.upper() - a.page_size()).max(a.lower())),
                        );
                    }
                }
            }
        });
        surface.covers[index].add_controller(swipe);
        let weak = Rc::downgrade(&surface);
        surface.maximizers[index].connect_clicked(move |_| {
            if let Some(s) = weak.upgrade() {
                s.maximize(index);
            }
        });
        let weak = Rc::downgrade(&surface);
        surface.minimizers[index].connect_clicked(move |_| {
            if let Some(s) = weak.upgrade() {
                s.minimize(index);
            }
        });
        for b in [&surface.headings[index], &surface.covers[index]] {
            let weak = Rc::downgrade(&surface);
            b.connect_clicked(move |_| {
                if let Some(s) = weak.upgrade() {
                    s.choose(index);
                }
            });
        }
        for (widget, resize) in [
            (&surface.headings[index], false),
            (&surface.handles[index], true),
            (&surface.covers[index], false),
        ] {
            let drag = gtk::GestureDrag::new();
            drag.set_propagation_phase(gtk::PropagationPhase::Capture);
            let origin = Rc::new(Cell::new(defaults()[index]));
            let pointer = Rc::new(Cell::new((0.0, 0.0)));
            let anchor = pointer.clone();
            let weak = Rc::downgrade(&surface);
            let start = origin.clone();
            drag.connect_drag_begin(move |_, x, y| {
                if let Some(s) = weak.upgrade() {
                    if s.arranging.get() {
                        s.transition.borrow_mut().take();
                        s.selected.set(index);
                        start.set(s.regions.borrow()[index]);
                        anchor.set((x, y));
                        s.layout();
                    }
                }
            });
            let weak = Rc::downgrade(&surface);
            let start = origin.clone();
            let anchor = pointer.clone();
            let preview = widget == &surface.covers[index];
            drag.connect_drag_update(move |gesture, dx, dy| {
                if let Some(s) = weak.upgrade() {
                    if s.arranging.get() && dx.abs() + dy.abs() > 8.0 {
                        gesture.set_state(gtk::EventSequenceState::Claimed);
                        s.swap_source.set(None);
                        let mut r = start.get();
                        let w = s.shell.width().max(1) as f64;
                        let h = s.shell.height().max(1) as f64;
                        if !s.advanced.get() {
                            let (x, y) = anchor.get();
                            s.drop_hover.set(drop_target(
                                &s.regions.borrow(),
                                &s.hidden.borrow(),
                                index,
                                r.x + (x + dx) / w,
                                r.y + (y + dy + if preview { 52.0 } else { 0.0 }) / h,
                            ));
                            s.status.set_text("DROP ON A SYSTEM TO SWAP POSITIONS");
                            s.layout();
                            return;
                        }
                        if resize {
                            r.w += dx / w;
                            r.h += dy / h;
                        } else {
                            r.x += dx / w;
                            r.y += dy / h;
                        }
                        s.regions.borrow_mut()[index] = r.bounded();
                        s.layout();
                    }
                }
            });
            let weak = Rc::downgrade(&surface);
            drag.connect_drag_end(move |_, dx, dy| {
                if let Some(s) = weak.upgrade() {
                    if s.arranging.get() && dx.abs() + dy.abs() > 8.0 {
                        if !s.advanced.get() {
                            if let Some(target) = s.drop_hover.take() {
                                s.regions.borrow_mut().swap(index, target);
                                // Basic swaps take effect immediately on drop.
                                s.transition.borrow_mut().take();
                                s.layout();
                                s.persist();
                            } else {
                                s.layout();
                            }
                            return;
                        }
                        let r = s.regions.borrow()[index].snap();
                        s.regions.borrow_mut()[index] = r;
                        s.layout();
                        s.persist();
                    }
                }
            });
            widget.add_controller(drag);
        }
    }
    let weak = Rc::downgrade(&surface);
    overview.connect_clicked(move |_| {
        if let Some(s) = weak.upgrade() {
            s.home();
        }
    });
    let weak = Rc::downgrade(&surface);
    mode_picker.connect_selected_notify(move |picker| {
        if let Some(s) = weak.upgrade() {
            s.advanced.set(picker.selected() == 1);
            s.swap_source.set(None);
            s.sync_controls();
            s.status.set_text(if s.advanced.get() {
                "ADVANCED / DRAG TO MOVE / ↘ TO RESIZE"
            } else {
                "BASIC / DRAG ONTO ANOTHER SYSTEM TO SWAP"
            });
            s.layout();
        }
    });
    let weak = Rc::downgrade(&surface);
    arrange.connect_clicked(move |_| {
        if let Some(s) = weak.upgrade() {
            s.toggle_arrange();
        }
    });
    let weak = Rc::downgrade(&surface);
    auto.connect_clicked(move |_| {
        if let Some(s) = weak.upgrade() {
            s.reflow();
            s.focused.set(None);
            s.animate();
            s.persist();
        }
    });
    let mut child = edits.first_child();
    let mut action = 0;
    while let Some(widget) = child {
        child = widget.next_sibling();
        let b = widget.downcast::<gtk::Button>().unwrap();
        let weak = Rc::downgrade(&surface);
        let op = action;
        action += 1;
        b.connect_clicked(move |_| {
            if let Some(s) = weak.upgrade() {
                let i = s.selected.get();
                let mut r = s.regions.borrow()[i];
                match op {
                    0 => r.x -= 0.025,
                    1 => r.x += 0.025,
                    2 => r.y -= 0.025,
                    3 => r.y += 0.025,
                    4 => {
                        r.w -= 0.025;
                        r.h -= 0.025;
                    }
                    _ => {
                        r.w += 0.025;
                        r.h += 0.025;
                    }
                }
                s.regions.borrow_mut()[i] = r.snap();
                s.layout();
                s.persist();
            }
        });
    }
    let weak = Rc::downgrade(&surface);
    surface.shell.add_tick_callback(move |_, _| {
        let Some(s) = weak.upgrade() else {
            return glib::ControlFlow::Break;
        };
        s.remember_input();
        if s.active.get()
            && (s.transition.borrow().is_some()
                || s.dimensions.get() != (s.viewport.width(), s.viewport.height()))
        {
            s.layout();
        }
        glib::ControlFlow::Continue
    });
    let navigate: Navigate = {
        let s = surface.clone();
        let single = single.clone();
        Rc::new(move |name| {
            if s.active.get() {
                match name {
                    "__home" => {
                        s.home();
                        return;
                    }
                    "__arrange" => {
                        s.toggle_arrange();
                        return;
                    }
                    "__auto" => {
                        s.focused.set(None);
                        s.reflow();
                        s.animate();
                        s.persist();
                        return;
                    }
                    _ => {}
                }
                if let Some(i) = PANELS.iter().position(|(id, _)| *id == name) {
                    s.choose(i);
                }
            } else {
                single.set_visible_child_name(name);
                if let Some(i) = PANELS.iter().position(|(route, _)| *route == name) {
                    let weak = Rc::downgrade(&s);
                    glib::idle_add_local_once(move || {
                        if let Some(s) = weak.upgrade() {
                            if let Some(input) = s.input_memory.borrow()[i]
                                .as_ref()
                                .and_then(|w| w.upgrade())
                            {
                                if !s.active.get() && input.is_mapped() && input.is_sensitive() {
                                    input.grab_focus();
                                }
                            }
                        }
                    });
                }
            }
        })
    };
    let mode: SetMode = {
        let s = surface;
        let single = single.clone();
        let host = host.clone();
        Rc::new(move |enabled| {
            if enabled == s.active.get() {
                return;
            }
            s.suspend_input();
            s.restored_input.set(None);
            if enabled {
                for (i, page) in s.pages.iter().enumerate() {
                    single.remove(page);
                    if let Ok(scroll) = page.clone().downcast::<gtk::ScrolledWindow>() {
                        scroll.set_hscrollbar_policy(gtk::PolicyType::Automatic);
                    }
                    s.interiors[i].put(page, 0.0, 0.0);
                }
                s.active.set(true);
                s.focused.set(None);
                s.status.set_text("ALL SYSTEMS / TAP TO ZOOM IN");
                host.set_visible_child_name("workspace");
                s.layout();
            } else {
                for (i, page) in s.pages.iter().enumerate() {
                    s.interiors[i].remove(page);
                    page.set_visible(true);
                    page.set_size_request(-1, -1);
                    page.set_can_target(true);
                    page.set_sensitive(true);
                    page.set_focusable(true);
                    single.add_named(page, Some(PANELS[i].0));
                }
                s.active.set(false);
                host.set_visible_child_name("single");
                single.set_visible_child_name("settings");
            }
        })
    };
    (host, navigate, mode)
}

#[cfg(test)]
mod tests {
    #[test]
    fn old_saved_layout_keeps_manual_arrangement_and_new_flag_roundtrips() {
        let json = serde_json::json!({"regions": super::defaults(),"hidden":vec![false;6]});
        let old: super::SavedLayout = serde_json::from_value(json).unwrap();
        assert!(!old.automatic);
        let new = super::SavedLayout {
            automatic: true,
            ..old
        };
        let decoded: super::SavedLayout =
            serde_json::from_str(&serde_json::to_string(&new).unwrap()).unwrap();
        assert!(decoded.automatic);
    }

    use super::*;
    #[test]
    fn offscreen_and_invalid_geometry_is_recoverable() {
        let r = Region {
            x: 2.0,
            y: -1.0,
            w: f64::NAN,
            h: 4.0,
        }
        .bounded();
        assert!(r.x >= 0.0 && r.x + r.w <= 1.0);
        assert_eq!(r.y, 0.0);
        assert_eq!(r.h, 1.0);
    }
    #[test]
    fn snap_and_serialization_preserve_arrangement() {
        let r = Region {
            x: 0.133,
            y: 0.211,
            w: 0.42,
            h: 0.37,
        }
        .snap();
        let bytes = serde_json::to_vec(&vec![r; 6]).unwrap();
        let restored: Vec<Region> = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(restored[0].x, 0.125);
        assert_eq!(restored.len(), 6);
    }
    #[test]
    fn automatic_layout_packs_panels_and_reserves_logo_space() {
        for count in 1..=PANELS.len() {
            let ids = (0..count).collect::<Vec<_>>();
            let slots = automatic(&ids);
            let panels = ids.iter().map(|&i| slots[i]).collect::<Vec<_>>();
            let logo = logo_region(&panels);
            let coverage = panels.iter().map(|r| r.w * r.h).sum::<f64>() + logo.w * logo.h;
            assert!(coverage > 0.95, "count {count}: {coverage}");
            assert!(logo.w * logo.h > 0.09);
            if count == 6 {
                let areas = panels.iter().map(|r| r.w * r.h).collect::<Vec<_>>();
                assert!(
                    areas.iter().copied().fold(f64::NEG_INFINITY, f64::max)
                        / areas.iter().copied().fold(f64::INFINITY, f64::min)
                        < 1.05
                );
            }
            for &i in &ids {
                let r = slots[i];

                assert!(r.x + r.w <= 1.0 && r.y + r.h <= 1.0);
            }
        }
    }
    #[test]
    fn swap_hit_testing_excludes_hidden_and_source() {
        let slots = defaults();
        let mut hidden = vec![false; PANELS.len()];
        assert_eq!(drop_target(&slots, &hidden, 0, 0.8, 0.1), Some(1));
        hidden[1] = true;
        assert_eq!(drop_target(&slots, &hidden, 0, 0.8, 0.1), None);
        assert_eq!(drop_target(&slots, &hidden, 0, 0.1, 0.1), None);
        assert_eq!(drop_target(&slots, &hidden, 0, 0.5, 0.5), None);
    }
    #[test]
    fn zoom_interpolation_reaches_both_endpoints() {
        let a = defaults()[1];
        let b = Region {
            x: 0.0,
            y: 0.0,
            w: 1.0,
            h: 1.0,
        };
        assert_eq!(interpolate(a, b, 0.0).x, a.x);
        assert_eq!(interpolate(a, b, 1.0).w, 1.0);
        assert!(interpolate(a, b, 0.5).w > a.w);
    }
    #[test]
    fn defaults_fit_without_overlap() {
        let d = defaults();
        for (i, a) in d.iter().enumerate() {
            assert!(a.x + a.w <= 1.0 && a.y + a.h <= 1.0);
            for b in &d[i + 1..] {
                assert!(
                    a.x + a.w <= b.x || b.x + b.w <= a.x || a.y + a.h <= b.y || b.y + b.h <= a.y
                );
            }
        }
    }
}
