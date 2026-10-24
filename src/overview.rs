// Mixed provenance: Codex overview baseline; Claude AI continuation (user-attributed)
// refined open-sided instruments and reference styling. See docs/WORK_PROVENANCE.md.
//! Home instruments lay out at native pixels. No full-page thumbnail scaling.
use super::InstrumentData;
use crate::services::Preferences;
use gtk::{cairo, glib, prelude::*};
use std::cell::{Cell, RefCell};
use std::rc::Rc;

#[derive(Default)]
pub(in crate::ui) struct Sources {
    pub files_path: Option<gtk::Entry>,
    pub files_list: Option<gtk::ListBox>,
    pub files_status: Option<gtk::Label>,
    pub file_details: Option<gtk::Label>,
    pub apps_list: Option<gtk::ListBox>,
    pub apps_status: Option<gtk::Label>,
    pub network_status: Option<gtk::Label>,
    pub network_report: Option<gtk::TextBuffer>,
    pub speed_result: Option<gtk::Label>,
    pub calculator: Option<gtk::Label>,
    pub diagnostics: Option<gtk::TextBuffer>,
}
const AMBER: (f64, f64, f64) = crate::theme::rgb(crate::theme::AMBER);
const LILAC: (f64, f64, f64) = crate::theme::rgb(crate::theme::LILAC);
const CORAL: (f64, f64, f64) = crate::theme::rgb(crate::theme::CORAL);
const RULE: (f64, f64, f64) = crate::theme::rgb(crate::theme::RULE);
const INK: (f64, f64, f64) = (0.0, 0.0, 0.0);
struct Paint<'a> {
    cr: &'a cairo::Context,
    width: f64,
    height: f64,
}
impl Paint<'_> {
    fn rect(&self, x: f64, y: f64, w: f64, h: f64, c: (f64, f64, f64)) {
        self.cr.set_source_rgb(c.0, c.1, c.2);
        self.cr.rectangle(x, y, w.max(0.0), h.max(0.0));
        let _ = self.cr.fill();
    }
    fn text(&self, x: f64, y: f64, size: f64, width: f64, text: &str, color: (f64, f64, f64)) {
        if y > self.height || width <= 0.0 {
            return;
        }
        self.cr.select_font_face(
            crate::theme::FONT,
            cairo::FontSlant::Normal,
            cairo::FontWeight::Bold,
        );
        self.cr.set_font_size(size);
        self.cr.set_source_rgb(color.0, color.1, color.2);
        let layout = shorten(text, width, |s| {
            self.cr
                .text_extents(s)
                .map(|e| e.x_advance())
                .unwrap_or(0.0)
        });
        self.cr.move_to(x, y);
        let _ = self.cr.show_text(&layout);
    }
    fn band(&self, y: f64, text: &str) {
        self.rect(8.0, y, self.width - 42.0, 22.0, LILAC);
        self.rect(self.width - 31.0, y, 10.0, 22.0, CORAL);
        self.rect(self.width - 18.0, y, 10.0, 22.0, AMBER);
        self.text(14.0, y + 16.0, 14.0, self.width - 54.0, text, INK);
    }
    fn pair(&self, y: f64, key: &str, value: &str) {
        let split = (self.width * 0.48).clamp(100.0, 240.0);
        self.text(10.0, y, 16.0, split - 16.0, key, CORAL);
        self.text(split, y, 18.0, self.width - split - 10.0, value, AMBER);
        self.rect(10.0, y + 7.0, self.width - 20.0, 1.0, (0.22, 0.15, 0.23));
    }
    fn lines(&self, y: f64, text: &str, size: f64, max_lines: usize) {
        let available = self.width - 20.0;
        self.cr.select_font_face(
            crate::theme::FONT,
            cairo::FontSlant::Normal,
            cairo::FontWeight::Bold,
        );
        self.cr.set_font_size(size);
        let lines = wrap(text, available, |s| {
            self.cr
                .text_extents(s)
                .map(|e| e.x_advance())
                .unwrap_or(0.0)
        });
        for (i, line) in lines.iter().take(max_lines).enumerate() {
            self.text(
                10.0,
                y + i as f64 * (size + 5.0),
                size,
                available,
                line,
                LILAC,
            );
        }
    }
}
fn shorten(text: &str, width: f64, measure: impl Fn(&str) -> f64) -> String {
    let clean = text.replace(['\n', '\r'], " ");
    if measure(&clean) <= width {
        return clean;
    }
    let mut chars: Vec<char> = clean.chars().collect();
    while !chars.is_empty() {
        chars.pop();
        let candidate = format!("{}…", chars.iter().collect::<String>());
        if measure(&candidate) <= width {
            return candidate;
        }
    }
    "…".into()
}
fn wrap(text: &str, width: f64, measure: impl Fn(&str) -> f64) -> Vec<String> {
    let mut lines = Vec::new();
    let mut line = String::new();
    for word in text.split_whitespace() {
        let next = if line.is_empty() {
            word.to_string()
        } else {
            format!("{line} {word}")
        };
        if !line.is_empty() && measure(&next) > width {
            lines.push(line);
            line = word.into();
        } else {
            line = next;
        }
    }
    if !line.is_empty() {
        lines.push(line);
    }
    lines
}
fn labels(widget: &gtk::Widget, out: &mut Vec<String>) {
    if let Some(l) = widget.downcast_ref::<gtk::Label>() {
        out.push(l.text().to_string());
        return;
    }
    let mut child = widget.first_child();
    while let Some(w) = child {
        child = w.next_sibling();
        labels(&w, out);
    }
}
fn rows(list: &Option<gtk::ListBox>) -> Vec<Vec<String>> {
    let mut result = Vec::new();
    if let Some(list) = list {
        let mut child = list.first_child();
        while let Some(w) = child {
            child = w.next_sibling();
            if w.property::<bool>("visible") {
                let mut text = Vec::new();
                labels(&w, &mut text);
                result.push(text);
            }
        }
    }
    result
}
fn text(label: &Option<gtk::Label>, fallback: &str) -> String {
    label
        .as_ref()
        .map(|l| l.text().to_string())
        .unwrap_or_else(|| fallback.into())
}
fn report(buffer: &Option<gtk::TextBuffer>) -> String {
    buffer
        .as_ref()
        .map(|b| b.text(&b.start_iter(), &b.end_iter(), false).to_string())
        .unwrap_or_default()
}
fn table_capacity(width: f64, height: f64, top: f64, apps: bool) -> (usize, usize, f64) {
    let columns = if apps && width >= 440.0 { 2 } else { 1 };
    let row_h = if height >= 250.0 { 32.0 } else { 28.0 };
    (
        columns,
        ((height - top - 26.0) / row_h).floor().max(1.0) as usize,
        row_h,
    )
}
fn draw_table(p: &Paint<'_>, source: &Sources, index: usize, offset: &Cell<usize>) {
    let apps = index == 3;
    let entries = rows(if apps {
        &source.apps_list
    } else {
        &source.files_list
    });
    let header = if apps {
        format!("{} APPLICATIONS / FEDORA", entries.len())
    } else {
        source
            .files_path
            .as_ref()
            .map(|e| e.text().to_string())
            .unwrap_or_else(|| "LOADING DIRECTORY".into())
    };
    p.text(10.0, 25.0, 20.0, p.width - 20.0, &header, AMBER);
    p.band(
        38.0,
        if apps {
            "INSTALLED APPLICATIONS"
        } else {
            "NAME"
        },
    );
    if !apps && p.width >= 310.0 {
        p.text(p.width - 102.0, 54.0, 14.0, 65.0, "DETAILS", INK);
    }
    let (columns, count, row_h) = table_capacity(p.width, p.height, 66.0, apps);
    let start = offset
        .get()
        .min(entries.len().saturating_sub(count * columns));
    offset.set(start);
    let cw = (p.width - 20.0) / columns as f64;
    for col in 0..columns {
        for row in 0..count {
            let Some(item) = entries.get(start + col * count + row) else {
                break;
            };
            let x = 10.0 + col as f64 * cw;
            let y = 84.0 + row as f64 * row_h;
            p.rect(x, y - 12.0, 5.0, 12.0, if apps { CORAL } else { AMBER });
            let name = item.first().map(String::as_str).unwrap_or("—");
            let details = item.get(1).map(String::as_str).unwrap_or("");
            let detail_width = if !apps && p.width >= 310.0 { 92.0 } else { 0.0 };
            p.text(x + 13.0, y, 18.0, cw - 18.0 - detail_width, name, AMBER);
            if detail_width > 0.0 {
                p.text(
                    x + cw - detail_width,
                    y,
                    14.0,
                    detail_width - 5.0,
                    details,
                    LILAC,
                );
            }
            p.rect(x, y + 7.0, cw - 8.0, 1.0, (0.26, 0.18, 0.27));
        }
    }
    let shown = (count * columns).min(entries.len().saturating_sub(start));
    let footer = if entries.is_empty() {
        if apps {
            text(&source.apps_status, "No applications")
        } else {
            text(&source.files_status, "Loading…")
        }
    } else if entries.len() > shown {
        format!(
            "{}–{} / {} • SCROLL FOR MORE",
            start + 1,
            start + shown,
            entries.len()
        )
    } else {
        format!("{} ITEMS / TOUCH TO OPEN SYSTEM", entries.len())
    };
    p.text(10.0, p.height - 5.0, 13.0, p.width - 20.0, &footer, LILAC);
}
fn draw_system(p: &Paint<'_>, data: &InstrumentData) {
    let clock = glib::DateTime::now_local()
        .and_then(|t| t.format("%H:%M:%S"))
        .map(|v| v.to_string())
        .unwrap_or_default();
    p.text(10.0, 34.0, 32.0, p.width - 20.0, &clock, AMBER);
    let Some(s) = &data.snapshot else {
        p.lines(64.0, "ACQUIRING TELEMETRY", 18.0, 2);
        return;
    };
    p.text(
        10.0,
        57.0,
        15.0,
        p.width - 20.0,
        &format!(
            "UPTIME {:02}h {:02}m  /  FEDORA",
            s.uptime / 3600,
            s.uptime % 3600 / 60
        ),
        CORAL,
    );
    let graph_h = ((p.height - 180.0) * 0.45).max(28.0);
    let gy = 70.0;
    for i in 1..=2 {
        p.rect(
            14.0,
            gy + graph_h * i as f64 / 3.0,
            p.width - 24.0,
            1.0,
            RULE,
        );
    }
    p.rect(10.0, gy, 3.0, graph_h, LILAC);
    p.rect(10.0, gy + graph_h, p.width - 20.0, 2.0, LILAC);
    p.cr.set_source_rgb(LILAC.0, LILAC.1, LILAC.2);
    p.cr.set_line_width(1.5);
    for (i, value) in data.history.iter().enumerate() {
        let x = 10.0 + (60 - data.history.len() + i) as f64 * (p.width - 20.0) / 59.0;
        let y = gy + graph_h * (1.0 - value / 100.0);
        if i == 0 {
            p.cr.move_to(x, y)
        } else {
            p.cr.line_to(x, y)
        }
    }
    let _ = p.cr.stroke();
    let memory = (s.memory_total > 0).then(|| s.memory_used as f64 / s.memory_total as f64 * 100.0);
    let disk = s
        .storage
        .split_whitespace()
        .last()
        .and_then(|v| v.strip_suffix('%'))
        .and_then(|v| v.parse::<f64>().ok());
    let top = gy + graph_h + 12.0;
    let cell_w = (p.width - 20.0) / 3.0;
    let bar_h = (p.height - top - if p.height > 320.0 { 82.0 } else { 62.0 }).max(20.0);
    for (i, (name, value)) in [("CPU", Some(s.cpu)), ("RAM", memory), ("HOME", disk)]
        .iter()
        .enumerate()
    {
        let x = 10.0 + i as f64 * cell_w;
        p.text(
            x,
            top + 18.0,
            22.0,
            cell_w - 4.0,
            &value
                .map(|v| format!("{v:.0}%"))
                .unwrap_or_else(|| "—".into()),
            AMBER,
        );
        let gauge_w = (cell_w * 0.56).clamp(18.0, 56.0);
        let gx = x + (cell_w - 10.0 - gauge_w) * 0.5;
        let gy0 = top + 26.0;
        // Open-sided instrument: a spine and a foot, never an enclosing outline.
        p.rect(gx - 6.0, gy0, 3.0, bar_h, LILAC);
        p.rect(gx - 6.0, gy0 + bar_h, gauge_w + 6.0, 3.0, LILAC);
        let segments = ((bar_h / 9.0).floor() as usize).clamp(4, 14);
        let seg_h = (bar_h - 2.0 * (segments - 1) as f64) / segments as f64;
        let lit = value
            .map(|v| (v.clamp(0.0, 100.0) / 100.0 * segments as f64).round() as usize)
            .unwrap_or(0);
        for seg in 0..segments {
            let sy = gy0 + bar_h - (seg as f64 + 1.0) * seg_h - seg as f64 * 2.0;
            let color = if seg >= lit {
                RULE
            } else if i == 0 {
                AMBER
            } else {
                CORAL
            };
            p.rect(gx, sy, gauge_w, seg_h, color);
        }
        p.text(x, top + bar_h + 45.0, 16.0, cell_w - 4.0, name, LILAC);
    }
    if p.height > 320.0 {
        p.text(
            10.0,
            p.height - 5.0,
            15.0,
            p.width - 20.0,
            &format!(
                "MEMORY {:.1} / {:.1} GiB",
                s.memory_used as f64 / 1073741824.0,
                s.memory_total as f64 / 1073741824.0
            ),
            LILAC,
        );
    }
}
fn draw_network(p: &Paint<'_>, data: &InstrumentData) {
    let state = text(&data.sources.network_status, "CHECKING");
    p.text(
        10.0,
        31.0,
        27.0,
        p.width - 20.0,
        state.split('/').next().unwrap_or(&state),
        AMBER,
    );
    let Some(s) = &data.snapshot else { return };
    p.band(44.0, "SUBSPACE LINK / LIVE TRAFFIC");
    p.pair(
        92.0,
        "RECEIVING",
        &format!("{:.1} KiB/s", s.received as f64 / 1024.0),
    );
    p.pair(
        124.0,
        "SENDING",
        &format!("{:.1} KiB/s", s.transmitted as f64 / 1024.0),
    );
    let report_top = if p.height > 340.0 {
        p.band(145.0, "ADAPTERS / ROUTES / DIAGNOSTICS");
        let r = report(&data.sources.network_report);
        let lines = if r.starts_with("Run diagnostics") {
            "Select Network to inspect adapters, routes, and DNS. No connection will be changed."
                .to_string()
        } else {
            r.lines()
                .filter(|line| !line.is_empty())
                .take(8)
                .collect::<Vec<_>>()
                .join(" / ")
        };
        p.lines(187.0, &lines, 17.0, 3);
        if p.height > 420.0 {
            p.band(250.0, "TRAFFIC HISTORY / RECEIVE + SEND");
            let top = 284.0;
            let graph_h = (p.height - top - 108.0).max(20.0);
            let peak = data
                .traffic
                .iter()
                .map(|(rx, tx)| rx.max(*tx))
                .fold(1.0, f64::max);
            for i in 0..=3 {
                p.rect(
                    10.0,
                    top + graph_h * i as f64 / 3.0,
                    p.width - 20.0,
                    1.0,
                    (0.2, 0.16, 0.25),
                );
            }
            for (send, color) in [(false, AMBER), (true, LILAC)] {
                p.cr.set_source_rgb(color.0, color.1, color.2);
                p.cr.set_line_width(1.5);
                for (i, (rx, tx)) in data.traffic.iter().enumerate() {
                    let x = 10.0 + (60 - data.traffic.len() + i) as f64 * (p.width - 20.0) / 59.0;
                    let y = top + graph_h * (1.0 - if send { tx } else { rx } / peak);
                    if i == 0 {
                        p.cr.move_to(x, y);
                    } else {
                        p.cr.line_to(x, y);
                    }
                }
                let _ = p.cr.stroke();
            }
            p.text(
                12.0,
                top + 16.0,
                13.0,
                p.width - 24.0,
                &format!("SCALE 0–{peak:.1} KiB/s"),
                CORAL,
            );
        }
        p.height - 90.0
    } else {
        142.0
    };
    if p.height > 180.0 {
        p.band(report_top, "TRANSFER TEST");
        p.lines(
            report_top + 43.0,
            &text(&data.sources.speed_result, "READY / no test has run"),
            17.0,
            ((p.height - report_top - 44.0) / 22.0).max(1.0) as usize,
        );
    }
}
fn draw_tools(p: &Paint<'_>, source: &Sources) {
    p.text(
        10.0,
        24.0,
        15.0,
        p.width - 20.0,
        "CALCULATOR / LAST RESULT",
        CORAL,
    );
    p.text(
        10.0,
        65.0,
        36.0,
        p.width - 20.0,
        &text(&source.calculator, "0"),
        AMBER,
    );
    if p.height < 170.0 {
        p.band(76.0, "SYSTEM / PROCESS DIAGNOSTICS");
        let r = report(&source.diagnostics);
        p.lines(
            119.0,
            if r.starts_with("Read-only scan") {
                "No scan yet • select Utilities"
            } else {
                "Report available • select Utilities"
            },
            16.0,
            1,
        );
        return;
    }
    let width = (p.width - 20.0) / 4.0;
    for (i, name) in ["+", "−", "×", "÷"].iter().enumerate() {
        let x = 10.0 + i as f64 * width;
        p.rect(
            x,
            78.0,
            width - 4.0,
            28.0,
            if i % 2 == 0 { LILAC } else { CORAL },
        );
        p.text(x + width / 2.0 - 6.0, 99.0, 22.0, width - 8.0, name, INK);
    }
    if p.height > 143.0 {
        p.band(118.0, "SYSTEM / PROCESS DIAGNOSTICS");
        let report = report(&source.diagnostics);
        let message = if report.starts_with("Read-only scan") {
            "No scan yet • select Utilities to inspect processes".into()
        } else {
            report
                .lines()
                .filter(|s| !s.is_empty())
                .take(3)
                .collect::<Vec<_>>()
                .join(" / ")
        };
        p.lines(
            161.0,
            &message,
            17.0,
            ((p.height - 161.0) / 22.0).max(1.0) as usize,
        );
    }
}
fn accessible_summary(index: usize, data: &InstrumentData, prefs: &Preferences) -> String {
    match index {
        0 => data
            .snapshot
            .as_ref()
            .map(|s| {
                format!(
                    "CPU {:.0} percent. Memory {:.1} of {:.1} GiB. Uptime {} minutes.",
                    s.cpu,
                    s.memory_used as f64 / 1073741824.0,
                    s.memory_total as f64 / 1073741824.0,
                    s.uptime / 60
                )
            })
            .unwrap_or_else(|| "Acquiring telemetry".into()),
        1 | 3 => {
            let source = &data.sources;
            let entries = rows(if index == 1 {
                &source.files_list
            } else {
                &source.apps_list
            });
            let names = entries
                .iter()
                .take(5)
                .filter_map(|row| row.first())
                .cloned()
                .collect::<Vec<_>>()
                .join(", ");
            let location = if index == 1 {
                source
                    .files_path
                    .as_ref()
                    .map(|p| p.text().to_string())
                    .unwrap_or_default()
            } else {
                "Installed applications".into()
            };
            format!(
                "{location}. {} items. {names}. Open system for full controls.",
                entries.len()
            )
        }
        2 => format!(
            "{}. Transfer test: {}",
            text(&data.sources.network_status, "Checking connectivity"),
            text(&data.sources.speed_result, "Ready")
        ),
        4 => format!(
            "Calculator result {}. Open Utilities for calculator and process diagnostics.",
            text(&data.sources.calculator, "0")
        ),
        _ => format!(
            "Workspace {}. Alert sound {}. Open Preferences to configure.",
            if prefs.workspace_mode {
                "Console"
            } else {
                "Desktop"
            },
            if prefs.sound { "on" } else { "off" }
        ),
    }
}
pub(super) struct Overview {
    pub widget: gtk::ScrolledWindow,
    pub canvas: gtk::DrawingArea,
    pub offset: Rc<Cell<usize>>,
}
pub(super) fn build(
    index: usize,
    data: Rc<RefCell<InstrumentData>>,
    prefs: Rc<RefCell<Preferences>>,
    activator: &gtk::Button,
) -> Overview {
    let summary_data = data.clone();
    let summary_prefs = prefs.clone();
    let weak_activator = activator.downgrade();
    let name = format!(
        "Open {}",
        lcars_desktop::workspace::PanelId::ALL[index].title()
    );
    activator.update_property(&[gtk::accessible::Property::Label(&name)]);
    let mut last_summary = String::new();
    let widget = gtk::DrawingArea::new();
    widget.set_can_target(false);
    let offset = Rc::new(Cell::new(0));
    let scroll = offset.clone();
    widget.set_draw_func(move |_, cr, w, h| {
        let p = Paint {
            cr,
            width: w as f64,
            height: h as f64,
        };
        let data = data.borrow();
        match index {
            0 => draw_system(&p, &data),
            1 | 3 => draw_table(&p, &data.sources, index, &scroll),
            2 => draw_network(&p, &data),
            4 => draw_tools(&p, &data.sources),
            _ => {
                p.band(6.0, "CONSOLE CONFIGURATION");
                p.pair(
                    55.0,
                    "WORKSPACE",
                    if prefs.borrow().workspace_mode {
                        "CONSOLE"
                    } else {
                        "DESKTOP"
                    },
                );
                p.pair(
                    87.0,
                    "ALERT SOUND",
                    if prefs.borrow().sound { "ON" } else { "OFF" },
                );
                if p.height > 130.0 {
                    p.pair(119.0, "LAYOUT", "START / ARRANGE");
                }
                if p.height > 164.0 {
                    p.pair(151.0, "DISPLAY", "FULLSCREEN");
                }
                if p.height > 196.0 {
                    p.lines(186.0, "Touch to configure the console", 16.0, 2);
                }
            }
        }
    });
    let weak = widget.downgrade();
    glib::timeout_add_local(std::time::Duration::from_millis(500), move || {
        let Some(w) = weak.upgrade() else {
            return glib::ControlFlow::Break;
        };
        if w.is_mapped() {
            w.queue_draw();
            let summary =
                accessible_summary(index, &summary_data.borrow(), &summary_prefs.borrow());
            if summary != last_summary {
                w.update_property(&[gtk::accessible::Property::Label(&summary)]);
                if let Some(button) = weak_activator.upgrade() {
                    button.update_property(&[gtk::accessible::Property::Description(&summary)]);
                }
                last_summary = summary;
            }
        }
        glib::ControlFlow::Continue
    });
    let scroll = gtk::ScrolledWindow::new();
    scroll.set_policy(gtk::PolicyType::Never, gtk::PolicyType::Automatic);
    scroll.set_child(Some(&widget));
    widget.set_hexpand(true);
    widget.set_vexpand(true);
    widget.set_content_height(match index {
        0 => 198,
        1 | 3 => 150,
        2 => 190,
        4 => 128,
        _ => 140,
    });
    Overview {
        widget: scroll,
        canvas: widget,
        offset,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn text_truncation_preserves_unicode() {
        assert_eq!(shorten("αβγδε", 4.0, |s| s.chars().count() as f64), "αβγ…");
    }
    #[test]
    fn app_columns_respond_to_width() {
        assert_eq!(table_capacity(300.0, 300.0, 66.0, true).0, 1);
        assert_eq!(table_capacity(500.0, 300.0, 66.0, true).0, 2);
    }
    #[test]
    fn wrapping_keeps_words() {
        assert_eq!(
            wrap("live network traffic", 12.0, |s| s.len() as f64),
            vec!["live network", "traffic"]
        );
    }
}
