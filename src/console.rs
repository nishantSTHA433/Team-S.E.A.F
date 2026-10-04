use super::{button, column, draw_emblem, label, row, Navigate, SetMode, PANELS};
use crate::services::{self, Preferences};
use gtk::{glib, prelude::*};
use serde::{Deserialize, Serialize};
use std::cell::{Cell, RefCell};
use std::rc::Rc;

#[derive(Default)]
pub(super) struct InstrumentData {
    snapshot: Option<services::Snapshot>,
    history: std::collections::VecDeque<f64>,
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
    }
}
fn system_instrument(data: Rc<RefCell<InstrumentData>>) -> gtk::DrawingArea {
    let instrument = gtk::DrawingArea::new();
    instrument.set_can_target(false);
    instrument.set_draw_func(move |_, cr, width, height| {
        let scale = (width as f64 / 320.0).min(height as f64 / 200.0);
        let _ = cr.save();
        cr.scale(scale, scale);
        let coral = (0.80, 0.45, 0.49);
        let lavender = (0.66, 0.61, 0.83);
        let amber = (1.0, 0.77, 0.34);
        let text = |x: f64, y: f64, size: f64, content: &str, color: (f64, f64, f64)| {
            cr.set_source_rgb(color.0, color.1, color.2);
            cr.select_font_face(
                "Nimbus Sans Narrow",
                gtk::cairo::FontSlant::Normal,
                gtk::cairo::FontWeight::Bold,
            );
            cr.set_font_size(size);
            cr.move_to(x, y);
            let _ = cr.show_text(content);
        };
        // The clock and instrumentation are drawn at the overview's actual size.
        cr.set_source_rgb(lavender.0, lavender.1, lavender.2);
        cr.rectangle(0.0, 0.0, 29.0, 44.0);
        let _ = cr.fill();
        text(7.0, 18.0, 12.0, "047", (0.0, 0.0, 0.0));
        text(7.0, 35.0, 11.0, "SYS", (0.0, 0.0, 0.0));
        let now = glib::DateTime::now_local().ok();
        let clock = now
            .as_ref()
            .and_then(|d| d.format("%H:%M:%S").ok())
            .map(|v| v.to_string())
            .unwrap_or_else(|| "--:--:--".into());
        let date = now
            .as_ref()
            .and_then(|d| d.format("%d.%m.%Y").ok())
            .map(|v| v.to_string())
            .unwrap_or_default();
        text(40.0, 35.0, 35.0, &clock, amber);
        text(202.0, 16.0, 12.0, "LCARS TIME / SYSTEM 01", amber);
        text(202.0, 35.0, 16.0, &date, coral);
        for (x, w, color) in [
            (0.0, 28.0, lavender),
            (32.0, 138.0, coral),
            (174.0, 88.0, lavender),
            (266.0, 54.0, amber),
        ] {
            cr.set_source_rgb(color.0, color.1, color.2);
            cr.rectangle(x, 48.0, w, 5.0);
            let _ = cr.fill();
        }
        cr.set_source_rgb(coral.0, coral.1, coral.2);
        cr.rectangle(0.0, 60.0, 28.0, 137.0);
        let _ = cr.fill();
        text(5.0, 78.0, 10.0, "CORE", (0.0, 0.0, 0.0));
        let data = data.borrow();
        let Some(s) = data.snapshot.as_ref() else {
            text(40.0, 93.0, 20.0, "ACQUIRING TELEMETRY", lavender);
            let _ = cr.restore();
            return;
        };
        let stale = data.updated.is_none_or(|t| t.elapsed().as_secs() > 20);
        text(
            40.0,
            70.0,
            12.0,
            if stale {
                "TELEMETRY STALE"
            } else {
                "FEDORA / LIVE CPU HISTORY"
            },
            coral,
        );
        cr.set_source_rgb(0.20, 0.17, 0.27);
        cr.set_line_width(0.5);
        for i in 0..=4 {
            let y = 79.0 + i as f64 * 8.0;
            cr.move_to(40.0, y);
            cr.line_to(199.0, y);
        }
        let _ = cr.stroke();
        cr.set_source_rgb(0.79, 0.77, 0.85);
        cr.set_line_width(1.0);
        for (i, value) in data.history.iter().enumerate() {
            let x = 40.0 + (60 - data.history.len() + i) as f64 * 159.0 / 59.0;
            let y = 111.0 - value * 0.32;
            if i == 0 {
                cr.move_to(x, y)
            } else {
                cr.line_to(x, y)
            }
        }
        let _ = cr.stroke();
        text(213.0, 76.0, 11.0, "UPTIME", coral);
        text(
            213.0,
            94.0,
            18.0,
            &format!("{:02}h {:02}m", s.uptime / 3600, s.uptime % 3600 / 60),
            amber,
        );
        let disk = s
            .storage
            .split_whitespace()
            .last()
            .and_then(|v| v.strip_suffix('%'))
            .and_then(|v| v.parse::<f64>().ok());
        let memory =
            (s.memory_total > 0).then(|| s.memory_used as f64 / s.memory_total as f64 * 100.0);
        for (i, (name, amount)) in [
            ("CPU", (!s.error.contains("CPU")).then_some(s.cpu)),
            ("RAM", memory),
            ("HOME", disk),
        ]
        .iter()
        .enumerate()
        {
            let x = 43.0 + i as f64 * 53.0;
            text(
                x,
                129.0,
                13.0,
                &amount
                    .map(|v| format!("{v:.0}%"))
                    .unwrap_or_else(|| "--".into()),
                lavender,
            );
            cr.set_source_rgb(coral.0, coral.1, coral.2);
            cr.set_line_width(0.8);
            cr.rectangle(x, 136.0, 20.0, 43.0);
            let _ = cr.stroke();
            if let Some(amount) = amount {
                let fill = amount.clamp(0.0, 100.0) * 0.41;
                cr.set_source_rgb(
                    if i == 0 { amber.0 } else { coral.0 },
                    if i == 0 { amber.1 } else { coral.1 },
                    if i == 0 { amber.2 } else { coral.2 },
                );
                cr.rectangle(x + 1.0, 178.0 - fill, 18.0, fill);
                let _ = cr.fill();
            }
            text(x, 194.0, 13.0, name, amber);
        }
        text(213.0, 125.0, 11.0, "RECEIVE / KiB/s", coral);
        text(
            213.0,
            145.0,
            20.0,
            &format!("{:.1}", s.received as f64 / 1024.0),
            amber,
        );
        text(213.0, 164.0, 11.0, "TRANSMIT / KiB/s", coral);
        text(
            213.0,
            187.0,
            20.0,
            &format!("{:.1}", s.transmitted as f64 / 1024.0),
            amber,
        );
        let _ = cr.restore();
    });
    let weak = instrument.downgrade();
    glib::timeout_add_local(std::time::Duration::from_secs(1), move || {
        let Some(widget) = weak.upgrade() else {
            return glib::ControlFlow::Break;
        };
        if widget.is_visible() {
            widget.queue_draw();
        }
        glib::ControlFlow::Continue
    });
    instrument
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
fn defaults() -> Vec<Region> {
    vec![
        Region {
            x: 0.0,
            y: 0.0,
            w: 0.30,
            h: 0.44,
        },
        Region {
            x: 0.65,
            y: 0.0,
            w: 0.35,
            h: 0.46,
        },
        Region {
            x: 0.65,
            y: 0.51,
            w: 0.35,
            h: 0.46,
        },
        Region {
            x: 0.0,
            y: 0.51,
            w: 0.30,
            h: 0.46,
        },
        Region {
            x: 0.33,
            y: 0.0,
            w: 0.29,
            h: 0.27,
        },
        Region {
            x: 0.33,
            y: 0.72,
            w: 0.29,
            h: 0.25,
        },
    ]
}
fn layout_path() -> std::path::PathBuf {
    std::env::var_os("XDG_CONFIG_HOME")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| services::home().join(".config"))
        .join("lcars-desktop/console-layout.json")
}
fn load() -> Vec<Region> {
    std::fs::read(layout_path())
        .ok()
        .and_then(|b| serde_json::from_slice::<Vec<Region>>(&b).ok())
        .filter(|v| v.len() == PANELS.len())
        .map(|v| v.into_iter().map(Region::bounded).collect())
        .unwrap_or_else(defaults)
}
fn save(regions: &[Region]) -> Result<(), String> {
    let path = layout_path();
    std::fs::create_dir_all(path.parent().ok_or("Missing configuration directory")?)
        .map_err(|e| e.to_string())?;
    let bytes = serde_json::to_vec_pretty(regions).map_err(|e| e.to_string())?;
    let temp = path.with_extension("tmp");
    std::fs::write(&temp, bytes).map_err(|e| e.to_string())?;
    std::fs::rename(temp, path).map_err(|e| e.to_string())
}
struct Surface {
    shell: gtk::Overlay,
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
    selected: Cell<usize>,
    arranging: Cell<bool>,
    active: Cell<bool>,
    dimensions: Cell<(i32, i32)>,
    status: gtk::Label,
    system_instrument: gtk::DrawingArea,
}
impl Surface {
    fn persist(&self) {
        self.status.set_text(&match save(&self.regions.borrow()) {
            Ok(()) => "ARRANGEMENT SAVED".into(),
            Err(e) => format!("SAVE FAILED / {e}"),
        });
    }
    fn layout(&self) {
        let width = self.shell.width().max(1) as f64;
        let height = self.shell.height().max(1) as f64;
        self.dimensions.set((width as i32, height as i32));
        let focus = self.focused.get();
        for (i, (_, title)) in PANELS.iter().enumerate() {
            let r = if let Some(f) = focus {
                if i == f {
                    Region {
                        x: 0.0,
                        y: 0.0,
                        w: 1.0,
                        h: 1.0,
                    }
                } else {
                    self.tiles[i].set_visible(false);
                    continue;
                }
            } else {
                self.regions.borrow()[i]
            };
            self.tiles[i].set_visible(true);
            let w = (r.w * width - 6.0).max(120.0) as i32;
            let h = (r.h * height - 6.0).max(105.0) as i32;
            self.tiles[i].set_size_request(w, h);
            self.canvas.move_(&self.tiles[i], r.x * width, r.y * height);
            let content_h = (h - 52).max(1);
            let raw_w = if focus.is_some() { w } else { 960 };
            let raw_h = if focus.is_some() { content_h } else { 640 };
            let overview_instrument = i == 0 && focus.is_none();
            self.pages[i].set_visible(!overview_instrument);
            if i == 0 {
                self.system_instrument.set_visible(overview_instrument);
                self.system_instrument.set_size_request(w, content_h);
            }
            self.pages[i].set_size_request(raw_w, raw_h);
            self.pages[i].set_can_target(focus.is_some());
            self.pages[i].set_sensitive(focus.is_some());
            self.pages[i].set_focusable(focus.is_some());
            let scale = if focus.is_some() {
                1.0
            } else {
                (w as f32 / 960.0).min(content_h as f32 / 640.0)
            };
            self.interiors[i].set_child_transform(
                &self.pages[i],
                Some(&gtk::gsk::Transform::new().scale(scale, scale)),
            );
            self.covers[i].set_visible(focus.is_none());
            self.handles[i].set_visible(self.arranging.get());
            self.titles[i].set_text(&format!(
                "{:02} / {}{}",
                i + 1,
                title,
                if self.arranging.get() { " / MOVE" } else { "" }
            ));
            if self.arranging.get() && self.selected.get() == i {
                self.tiles[i].add_css_class("arrange-selected");
            } else {
                self.tiles[i].remove_css_class("arrange-selected");
            }
        }
        self.background.queue_draw();
    }
    fn choose(&self, index: usize) {
        self.selected.set(index);
        if !self.arranging.get() {
            self.focused.set(Some(index));
        }
        self.status.set_text(if self.arranging.get() {
            "DRAG THE TITLE / DRAG ↘ TO RESIZE"
        } else {
            "SYSTEM FOCUSED / OVERVIEW TO ZOOM OUT"
        });
        self.layout();
    }
}

pub(super) fn build(
    single: &gtk::Stack,
    _prefs: &Rc<RefCell<Preferences>>,
    data: Rc<RefCell<InstrumentData>>,
) -> (gtk::Stack, Navigate, SetMode) {
    let host = gtk::Stack::new();
    host.set_hexpand(true);
    host.set_vexpand(true);
    host.add_named(single, Some("single"));
    let console = column(6);
    console.add_css_class("console-surface");
    let command = row(6);
    let overview = button("ALL SYSTEMS", "primary");
    let arrange = button("ARRANGE", "secondary");
    let auto = button("AUTO FIT", "secondary");
    command.append(&overview);
    command.append(&arrange);
    command.append(&auto);
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
    console.append(&shell);
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
    for (_, title) in PANELS {
        let tile = gtk::Overlay::new();
        tile.add_css_class("console-region");
        tile.set_overflow(gtk::Overflow::Hidden);
        let bounds = gtk::DrawingArea::new();
        tile.set_child(Some(&bounds));
        let body = column(4);
        let heading = button("", "system-band");
        let band = row(3);
        band.set_valign(gtk::Align::Center);
        let title_text = label(title, "console-band-title");
        title_text.set_wrap(false);
        title_text.set_ellipsize(gtk::pango::EllipsizeMode::End);
        title_text.set_hexpand(true);
        band.append(&title_text);
        for class in ["band-coral", "band-lilac", "band-amber"] {
            let segment = column(0);
            segment.add_css_class(class);
            segment.set_width_request(12);
            band.append(&segment);
        }
        heading.set_child(Some(&band));
        titles.push(title_text);
        body.append(&heading);
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
    let system_instrument = system_instrument(data);
    interiors[0].put(&system_instrument, 0.0, 0.0);
    let surface = Rc::new(Surface {
        shell,
        canvas,
        background,
        regions: RefCell::new(load()),
        tiles,
        interiors,
        pages,
        headings,
        titles,
        covers,
        handles,
        focused: Cell::new(None),
        selected: Cell::new(0),
        arranging: Cell::new(false),
        active: Cell::new(false),
        dimensions: Cell::new((0, 0)),
        status,
        system_instrument,
    });
    let weak = Rc::downgrade(&surface);
    surface.background.set_draw_func(move |area, cr, w, h| {
        let Some(s) = weak.upgrade() else { return };
        // Pick the largest empty square on a sampled grid so the emblem avoids system regions.
        let mut best = (0.5, 0.5, 0.0);
        if s.focused.get().is_none() {
            for yi in 0..20 {
                for xi in 0..20 {
                    let x = (xi as f64 + 0.5) / 20.0;
                    let y = (yi as f64 + 0.5) / 20.0;
                    let mut radius = x.min(1.0 - x).min(y).min(1.0 - y);
                    for r in s.regions.borrow().iter() {
                        let dx = (r.x - x).max(x - (r.x + r.w)).max(0.0);
                        let dy = (r.y - y).max(y - (r.y + r.h)).max(0.0);
                        radius = radius.min(dx.max(dy));
                    }
                    if radius > best.2 {
                        best = (x, y, radius)
                    }
                }
            }
        }
        let size = (best.2 * 2.0 * (w.min(h)) as f64).min(280.0);
        if size > 18.0 {
            let _ = cr.save();
            cr.translate(
                best.0 * w as f64 - size / 2.0,
                best.1 * h as f64 - size / 2.0,
            );
            // Reserve the bottom quarter of the available gap for the reference status lockup.
            cr.translate(size * 0.11, 0.0);
            cr.scale(size * 0.78 / 160.0, size * 0.78 / 160.0);
            draw_emblem(area, cr, 160, 160);
            let _ = cr.restore();
            if size >= 90.0 {
                cr.select_font_face(
                    "Nimbus Sans Narrow",
                    gtk::cairo::FontSlant::Normal,
                    gtk::cairo::FontWeight::Bold,
                );
                for (text, baseline, font_size, color) in [
                    ("SYSTEM AVAILABLE", 0.88, 0.145, (0.70, 0.64, 0.89)),
                    ("STATION ID 1440–900", 1.0, 0.085, (0.80, 0.45, 0.49)),
                ] {
                    cr.set_font_size(size * font_size);
                    cr.set_source_rgb(color.0, color.1, color.2);
                    if let Ok(extent) = cr.text_extents(text) {
                        cr.move_to(
                            best.0 * w as f64 - extent.width() / 2.0 - extent.x_bearing(),
                            best.1 * h as f64 - size / 2.0 + size * baseline,
                        );
                        let _ = cr.show_text(text);
                    }
                }
            }
        }
    });
    for index in 0..PANELS.len() {
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
        ] {
            let drag = gtk::GestureDrag::new();
            drag.set_propagation_phase(gtk::PropagationPhase::Capture);
            let origin = Rc::new(Cell::new(defaults()[index]));
            let weak = Rc::downgrade(&surface);
            let start = origin.clone();
            drag.connect_drag_begin(move |gesture, _, _| {
                if let Some(s) = weak.upgrade() {
                    if s.arranging.get() {
                        s.selected.set(index);
                        start.set(s.regions.borrow()[index]);
                        gesture.set_state(gtk::EventSequenceState::Claimed);
                        s.layout();
                    }
                }
            });
            let weak = Rc::downgrade(&surface);
            let start = origin.clone();
            drag.connect_drag_update(move |_, dx, dy| {
                if let Some(s) = weak.upgrade() {
                    if s.arranging.get() {
                        let mut r = start.get();
                        let w = s.shell.width().max(1) as f64;
                        let h = s.shell.height().max(1) as f64;
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
            drag.connect_drag_end(move |_, _, _| {
                if let Some(s) = weak.upgrade() {
                    if s.arranging.get() {
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
    let edit_bar = edits.clone();
    let arrange_button = arrange.clone();
    overview.connect_clicked(move |_| {
        if let Some(s) = weak.upgrade() {
            s.focused.set(None);
            s.arranging.set(false);
            edit_bar.set_visible(false);
            arrange_button.set_label("ARRANGE");
            s.status.set_text("ALL SYSTEMS / TAP TO ZOOM IN");
            s.layout();
        }
    });
    let weak = Rc::downgrade(&surface);
    let edit_bar = edits.clone();
    arrange.connect_clicked(move |b| {
        if let Some(s) = weak.upgrade() {
            let on = !s.arranging.get();
            s.arranging.set(on);
            s.focused.set(None);
            edit_bar.set_visible(on);
            b.set_label(if on { "DONE" } else { "ARRANGE" });
            s.status.set_text(if on {
                "SELECT A SYSTEM / MOVE OR RESIZE"
            } else {
                "ALL SYSTEMS / TAP TO ZOOM IN"
            });
            if !on {
                s.persist();
            }
            s.layout();
        }
    });
    let weak = Rc::downgrade(&surface);
    auto.connect_clicked(move |_| {
        if let Some(s) = weak.upgrade() {
            *s.regions.borrow_mut() = defaults();
            s.focused.set(None);
            s.layout();
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
        if s.active.get() && s.dimensions.get() != (s.shell.width(), s.shell.height()) {
            s.layout();
        }
        glib::ControlFlow::Continue
    });
    let navigate: Navigate = {
        let s = surface.clone();
        let single = single.clone();
        Rc::new(move |name| {
            if s.active.get() {
                if let Some(i) = PANELS.iter().position(|(id, _)| *id == name) {
                    s.choose(i);
                }
            } else {
                single.set_visible_child_name(name);
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
