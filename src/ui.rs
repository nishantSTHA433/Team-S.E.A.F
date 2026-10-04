fn draw_emblem(_: &gtk::DrawingArea, cr: &gtk::cairo::Context, w: i32, h: i32) {
    let cx = w as f64 / 2.0;
    let cy = h as f64 / 2.0;
    cr.set_source_rgb(0.03, 0.03, 0.15);
    cr.arc(cx, cy, 52.0, 0.0, std::f64::consts::TAU);
    let _ = cr.fill();
    cr.set_source_rgb(0.67, 0.63, 0.82);
    cr.set_line_width(2.0);
    cr.arc(cx, cy, 54.0, 0.0, std::f64::consts::TAU);
    let _ = cr.stroke();
    for i in 0..36 {
        let angle = i as f64 * 2.39996;
        let radius = 8.0 + (i as f64 * 7.0) % 43.0;
        cr.arc(
            cx + angle.cos() * radius,
            cy + angle.sin() * radius,
            if i % 5 == 0 { 1.6 } else { 0.7 },
            0.0,
            std::f64::consts::TAU,
        );
        let _ = cr.fill();
    }
    // Paired leaf silhouettes frame the star field without raster scaling artifacts.
    cr.set_source_rgb(0.67, 0.66, 0.77);
    for side in [-1.0, 1.0] {
        for i in 0..9 {
            let a = -1.05 + i as f64 * 0.24;
            let x = cx + side * 65.0 * a.cos();
            let y = cy + 66.0 * a.sin();
            cr.move_to(x, y + 9.0);
            cr.curve_to(
                x + side * 13.0,
                y + 3.0,
                x + side * 10.0,
                y - 10.0,
                x + side * 8.0,
                y - 15.0,
            );
            cr.curve_to(x + side * 2.0, y - 6.0, x - side * 4.0, y - 2.0, x, y + 9.0);
            let _ = cr.fill();
        }
    }
    cr.move_to(cx, cy - 22.0);
    cr.line_to(cx + 5.0, cy - 4.0);
    cr.line_to(cx + 22.0, cy);
    cr.line_to(cx + 5.0, cy + 4.0);
    cr.line_to(cx, cy + 22.0);
    cr.line_to(cx - 5.0, cy + 4.0);
    cr.line_to(cx - 22.0, cy);
    cr.line_to(cx - 5.0, cy - 4.0);
    cr.close_path();
    let _ = cr.fill();
}

use crate::services::{self, Preferences};
use gtk::{gio, glib, prelude::*};
use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::path::PathBuf;
use std::rc::Rc;
use std::time::Duration;

fn label(text: &str, class: &str) -> gtk::Label {
    let l = gtk::Label::new(Some(text));
    l.set_xalign(0.0);
    l.add_css_class(class);
    l.set_wrap(true);
    l
}

fn column(spacing: i32) -> gtk::Box {
    gtk::Box::new(gtk::Orientation::Vertical, spacing)
}
fn row(spacing: i32) -> gtk::Box {
    gtk::Box::new(gtk::Orientation::Horizontal, spacing)
}

fn button(text: &str, class: &str) -> gtk::Button {
    let b = gtk::Button::with_label(text);
    b.add_css_class(class);
    b.set_height_request(48);
    b
}

fn page(title: &str, subtitle: &str) -> gtk::Box {
    let p = column(10);
    p.set_margin_top(10);
    p.set_margin_bottom(10);
    p.set_margin_start(24);
    p.set_margin_end(24);
    let titlebar = row(3);
    let heading = label(title, "page-title");
    heading.set_hexpand(true);
    titlebar.append(&heading);
    for class in [
        "title-segment-coral",
        "title-segment-lilac",
        "title-segment-amber",
    ] {
        let segment = column(0);
        segment.add_css_class(class);
        segment.set_width_request(20);
        titlebar.append(&segment);
    }
    p.append(&titlebar);
    p.append(&label(subtitle, "muted"));
    p
}

fn scrolled(widget: &impl IsA<gtk::Widget>) -> gtk::ScrolledWindow {
    gtk::ScrolledWindow::builder()
        .child(widget)
        .hexpand(true)
        .vexpand(true)
        .hscrollbar_policy(gtk::PolicyType::Never)
        .build()
}

fn card(title: &str) -> (gtk::Box, gtk::Label, gtk::ProgressBar) {
    let c = column(12);
    c.add_css_class("card");
    c.set_hexpand(true);
    c.add_css_class("telemetry-panel");
    c.append(&label(title, "instrument-title"));
    let value = label("—", "metric");
    let bar = gtk::ProgressBar::new();
    c.append(&value);
    c.append(&bar);
    (c, value, bar)
}

type Navigate = Rc<dyn Fn(&str)>;
type SetMode = Rc<dyn Fn(bool)>;
const PANELS: [(&str, &str); 6] = [
    ("dashboard", "SYSTEM"),
    ("files", "FILES"),
    ("network", "NETWORK"),
    ("apps", "APPLICATIONS"),
    ("tools", "UTILITIES"),
    ("settings", "PREFERENCES"),
];

#[path = "console.rs"]
mod console;

pub fn build(app: &gtk::Application) {
    let css = gtk::CssProvider::new();
    css.load_from_data(include_str!("../assets/lcars.css"));
    gtk::style_context_add_provider_for_display(
        &gtk::gdk::Display::default().expect("Display required"),
        &css,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title("LCARS • Fedora Operations")
        .decorated(false)
        .default_width(1080)
        .default_height(700)
        .build();
    let preferences = Rc::new(RefCell::new(Preferences::load()));
    let root = column(0);
    let header = row(5);
    header.add_css_class("masthead");
    let brand = label("LCARS 40282", "brand");
    let cap = column(0);
    cap.add_css_class("header-cap");
    cap.set_width_request(18);
    header.append(&cap);
    header.append(&brand);
    let sub = label("LIBRARY COMPUTER ACCESS / RETRIEVAL SYSTEM", "masthead-sub");
    sub.set_hexpand(true);
    header.append(&sub);
    let full = button("FULLSCREEN", "compact");
    let weak_window = window.downgrade();
    full.connect_clicked(move |_| {
        if let Some(w) = weak_window.upgrade() {
            if w.is_fullscreen() {
                w.unfullscreen();
            } else {
                w.fullscreen();
            }
        }
    });
    header.append(&full);
    let quit = button("EXIT", "compact");
    let weak_app = app.downgrade();
    quit.connect_clicked(move |_| {
        if let Some(a) = weak_app.upgrade() {
            a.quit();
        }
    });
    header.append(&quit);
    let end_cap = column(0);
    end_cap.add_css_class("header-end-cap");
    end_cap.set_width_request(18);
    header.append(&end_cap);
    root.append(&gtk::WindowHandle::builder().child(&header).build());
    let body = row(0);
    body.set_vexpand(true);
    let rail = column(3);
    rail.add_css_class("rail");
    rail.set_width_request(154);
    rail.append(&label("47\nCOMMAND", "rail-cap"));
    let stack = gtk::Stack::new();
    stack.set_hexpand(true);
    stack.set_vexpand(true);
    stack.set_transition_type(gtk::StackTransitionType::None);
    let dashboard = page(
        "SYSTEM OVERVIEW",
        "FEDORA / ARM64     •     LIVE GUEST TELEMETRY",
    );
    let hero = row(12);
    let hero_text = column(8);
    hero_text.set_hexpand(true);
    hero_text.append(&label("SYSTEM AVAILABLE", "hero"));
    hero_text.append(&label("STATION ID 1440–900  /  OPERATIONS ONLINE", "muted"));
    let emblem = gtk::DrawingArea::new();
    emblem.set_content_width(160);
    emblem.set_content_height(160);
    emblem.set_draw_func(draw_emblem);
    hero.append(&emblem);
    hero.append(&hero_text);
    let history = Rc::new(RefCell::new(VecDeque::<f64>::with_capacity(60)));
    let history_panel = column(6);
    history_panel.add_css_class("history-panel");
    history_panel.set_width_request(210);
    history_panel.append(&label("CPU / RECENT HISTORY", "instrument-title"));
    let plot = gtk::DrawingArea::new();
    plot.set_content_width(200);
    plot.set_content_height(88);
    plot.set_tooltip_text(Some(
        "CPU utilization: 0–100%. Up to 60 recent telemetry samples; newest at right.",
    ));
    let samples = history.clone();
    plot.set_draw_func(move |_, cr, w, h| {
        let width = (w - 2).max(1) as f64;
        let height = (h - 2).max(1) as f64;
        cr.set_source_rgb(0.20, 0.16, 0.28);
        cr.set_line_width(1.0);
        for i in 0..=4 {
            let y = 1.0 + height * i as f64 / 4.0;
            cr.move_to(1.0, y);
            cr.line_to(width + 1.0, y);
        }
        for i in 0..=10 {
            let x = 1.0 + width * i as f64 / 10.0;
            cr.move_to(x, 1.0);
            cr.line_to(x, height + 1.0);
        }
        let _ = cr.stroke();
        cr.set_source_rgb(1.0, 0.76, 0.31);
        cr.set_line_width(1.5);
        let values = samples.borrow();
        for (i, value) in values.iter().enumerate() {
            let x = 1.0 + width * (60 - values.len() + i) as f64 / 59.0;
            let y = 1.0 + height * (1.0 - value / 100.0);
            if i == 0 {
                cr.move_to(x, y);
            } else {
                cr.line_to(x, y);
            }
        }
        let _ = cr.stroke();
    });
    history_panel.append(&plot);
    history_panel.append(&label("0–100% / LIVE SAMPLES", "graph-scale"));
    hero.append(&history_panel);
    dashboard.append(&hero);
    let metrics = gtk::Grid::builder()
        .column_spacing(14)
        .row_spacing(14)
        .column_homogeneous(true)
        .build();
    let (cpu_card, cpu_value, cpu_bar) = card("01 / PROCESSOR LOAD");
    let (mem_card, mem_value, mem_bar) = card("02 / MEMORY UTILIZATION");
    let (net_card, net_value, net_bar) = card("03 / NETWORK RECEIVE");
    net_bar.set_visible(false);
    let (time_card, time_value, time_bar) = card("04 / SYSTEM UPTIME");
    time_bar.set_visible(false);
    metrics.attach(&cpu_card, 0, 0, 1, 1);
    metrics.attach(&mem_card, 1, 0, 1, 1);
    metrics.attach(&net_card, 0, 1, 1, 1);
    metrics.attach(&time_card, 1, 1, 1, 1);
    dashboard.append(&metrics);
    let storage = label("STORAGE / Reading filesystem…", "strip");
    dashboard.append(&storage);
    dashboard.append(&label("Virtual machine readings reflect Fedora, not the Mac host. Sensors not exposed by UTM are unavailable.", "muted"));
    stack.add_named(&scrolled(&dashboard), Some("dashboard"));
    stack.add_named(&scrolled(&files_page(&window)), Some("files"));
    let (network_page, network_status) = network_page();
    stack.add_named(&scrolled(&network_page), Some("network"));
    stack.add_named(&scrolled(&applications_page()), Some("apps"));
    stack.add_named(&scrolled(&crate::tools::page()), Some("tools"));
    let settings = settings_page(&preferences);
    let workspace_toggle =
        gtk::CheckButton::with_label("CONSOLE MODE — touch / zoom / arrange (off: Desktop mode)");
    workspace_toggle.set_height_request(48);
    workspace_toggle.set_active(preferences.borrow().workspace_mode);
    settings.prepend(&workspace_toggle);
    stack.add_named(&scrolled(&settings), Some("settings"));
    let (page_host, navigate_panel, set_mode) = console::build(&stack, &preferences);
    let mode_result = label("", "muted");
    settings.append(&mode_result);
    let cfg = preferences.clone();
    let apply_mode = set_mode.clone();
    workspace_toggle.connect_toggled(move |toggle| {
        apply_mode(toggle.is_active());
        cfg.borrow_mut().workspace_mode = toggle.is_active();
        mode_result.set_text(&match cfg.borrow().save() {
            Ok(()) => "Layout preference saved".into(),
            Err(e) => format!("Layout active, but could not save: {e}"),
        });
    });
    set_mode(preferences.borrow().workspace_mode);
    let nav_items = [
        ("01  OVERVIEW", "dashboard"),
        ("02  FILES", "files"),
        ("03  NETWORK", "network"),
        ("04  APPLICATIONS", "apps"),
        ("05  UTILITIES", "tools"),
        ("06  PREFERENCES", "settings"),
    ];
    let nav_buttons: Rc<RefCell<Vec<gtk::Button>>> = Rc::new(RefCell::new(Vec::new()));
    for (text, name) in nav_items {
        let b = button(text, "nav");
        if name == "dashboard" {
            b.add_css_class("selected");
        }
        let navigate = navigate_panel.clone();
        let buttons = nav_buttons.clone();
        b.connect_clicked(move |this| {
            navigate(name);
            for b in buttons.borrow().iter() {
                b.remove_css_class("selected");
            }
            this.add_css_class("selected");
        });
        nav_buttons.borrow_mut().push(b.clone());
        rail.append(&b);
    }
    let spacer = column(0);
    spacer.set_vexpand(true);
    rail.append(&spacer);
    rail.append(&label(
        "LOCAL SYSTEM\nFEDORA 38\nCONSOLE 0.2",
        "rail-footer",
    ));
    body.append(&rail);
    body.append(&page_host);
    root.append(&body);
    let footer = row(12);
    footer.add_css_class("footer");
    let indicator = label("● CHECKING NETWORK", "status");
    indicator.set_hexpand(true);
    let clock = label("", "console-clock");
    for (title, target) in [
        ("LCARS", "dashboard"),
        ("FILES", "files"),
        ("NETWORK", "network"),
        ("TOOLS", "tools"),
    ] {
        let shortcut = button(title, "dock-button");
        shortcut.set_height_request(32);
        let navigate = navigate_panel.clone();
        let buttons = nav_buttons.clone();
        shortcut.connect_clicked(move |_| {
            navigate(target);
            for b in buttons.borrow().iter() {
                b.remove_css_class("selected");
            }
            let index = match target {
                "dashboard" => 0,
                "files" => 1,
                "network" => 2,
                "tools" => 4,
                _ => 0,
            };
            if let Some(b) = buttons.borrow().get(index) {
                b.add_css_class("selected");
            }
        });
        footer.append(&shortcut);
    }
    footer.append(&indicator);
    footer.append(&clock);
    root.append(&footer);
    let alert = label("", "alert-banner");
    alert.set_visible(false);
    root.prepend(&alert);
    window.set_child(Some(&root));
    window.set_cursor_from_name(Some("default"));
    let motion = gtk::EventControllerMotion::new();
    let weak_cursor = window.downgrade();
    motion.connect_enter(move |_, _, _| {
        if let Some(w) = weak_cursor.upgrade() {
            w.set_cursor_from_name(Some("default"));
        }
    });
    window.add_controller(motion);
    let notification_app = app.clone();
    let network_indicator = indicator.clone();
    let ns = network_status.clone();
    let prefs = preferences.clone();
    let last_state = Rc::new(RefCell::new(String::new()));
    crate::network::subscribe(move |state| {
        network_indicator.set_text(&format!("● NETWORK {state}"));
        ns.set_text(&format!("{state} / NetworkManager live events"));
        let degraded = state != "ONLINE";
        alert.set_visible(degraded);
        alert.set_text(&format!("COMMUNICATIONS ALERT / {state}"));
        if degraded {
            network_indicator.add_css_class("warning");
        } else {
            network_indicator.remove_css_class("warning");
        }
        let mut previous = last_state.borrow_mut();
        if *previous != state {
            if !previous.is_empty() || degraded {
                let notification = gio::Notification::new(if degraded {
                    "LCARS: connection unavailable"
                } else {
                    "LCARS: connection restored"
                });
                notification.set_body(Some(&format!("Network state: {state}")));
                notification.set_priority(if degraded {
                    gio::NotificationPriority::Urgent
                } else {
                    gio::NotificationPriority::Normal
                });
                notification_app.send_notification(Some("lcars-network"), &notification);
                if prefs.borrow().sound {
                    std::thread::spawn(|| {
                        let _ = services::command("canberra-gtk-play", &["-i", "dialog-warning"]);
                    });
                }
            }
            let log_dir = services::home().join(".local/state/lcars-desktop");
            let _ = std::fs::create_dir_all(&log_dir);
            use std::io::Write;
            if let Ok(mut f) = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(log_dir.join("network-events.log"))
            {
                let _ = writeln!(
                    f,
                    "{} {state}",
                    glib::DateTime::now_local()
                        .and_then(|d| d.format("%Y-%m-%d %H:%M:%S"))
                        .map(|v| v.to_string())
                        .unwrap_or_default()
                );
            }
            *previous = state;
        }
    });
    let keys = gtk::EventControllerKey::new();
    let weak = window.downgrade();
    let shortcut_navigation = navigate_panel.clone();
    let shortcut_buttons = nav_buttons.clone();
    keys.connect_key_pressed(move |_, key, _, modifiers| {
        if modifiers.contains(gtk::gdk::ModifierType::ALT_MASK) {
            let target = match key {
                gtk::gdk::Key::_1 => Some("dashboard"),
                gtk::gdk::Key::_2 => Some("files"),
                gtk::gdk::Key::_3 => Some("network"),
                gtk::gdk::Key::_4 => Some("apps"),
                gtk::gdk::Key::_5 => Some("tools"),
                gtk::gdk::Key::_6 => Some("settings"),
                _ => None,
            };
            if let Some(target) = target {
                shortcut_navigation(target);
                for (index, button) in shortcut_buttons.borrow().iter().enumerate() {
                    if PANELS[index].0 == target {
                        button.add_css_class("selected");
                    } else {
                        button.remove_css_class("selected");
                    }
                }
                return glib::Propagation::Stop;
            }
        }
        if let Some(w) = weak.upgrade() {
            if key == gtk::gdk::Key::Escape {
                w.unfullscreen();
                return glib::Propagation::Stop;
            }
            if key == gtk::gdk::Key::F11 {
                if w.is_fullscreen() {
                    w.unfullscreen();
                } else {
                    w.fullscreen();
                }
                return glib::Propagation::Stop;
            }
        }
        glib::Propagation::Proceed
    });
    window.add_controller(keys);
    let adaptive_prefs = preferences.clone();
    let compact_hero = hero.clone();
    let dashboard_for_layout = dashboard.clone();
    let rx = services::monitor();
    let mut last_update = std::time::Instant::now();
    let weak = window.downgrade();
    glib::timeout_add_local(Duration::from_millis(250), move || {
        if weak.upgrade().is_none() {
            return glib::ControlFlow::Break;
        }
        let available_height = dashboard_for_layout
            .ancestor(gtk::ScrolledWindow::static_type())
            .map(|w| w.height())
            .unwrap_or(700);
        compact_hero
            .set_visible(!adaptive_prefs.borrow().workspace_mode || available_height >= 440);
        while let Ok(s) = rx.try_recv() {
            last_update = std::time::Instant::now();
            {
                let mut samples = history.borrow_mut();
                if samples.len() == 60 {
                    samples.pop_front();
                }
                samples.push_back(s.cpu.clamp(0.0, 100.0));
            }
            plot.queue_draw();
            cpu_value.set_text(&format!("{:.0}%", s.cpu));
            cpu_bar.set_fraction(s.cpu / 100.0);
            mem_value.set_text(&format!(
                "{:.1} / {:.1} GiB",
                s.memory_used as f64 / 1073741824.0,
                s.memory_total as f64 / 1073741824.0
            ));
            mem_bar.set_fraction(if s.memory_total > 0 {
                s.memory_used as f64 / s.memory_total as f64
            } else {
                0.0
            });
            net_value.set_text(&format!("{:.0} KiB/s", s.received as f64 / 1024.0));
            time_value.set_text(&format!(
                "{:02}h {:02}m",
                s.uptime / 3600,
                (s.uptime % 3600) / 60
            ));
            storage.set_text(&format!(
                "STORAGE / SIZE · USED · AVAILABLE · USE\n{}",
                s.storage
            ));
            if !s.error.is_empty() {
                indicator.set_tooltip_text(Some(&s.error));
            }
        }
        clock.set_text(&format!(
            "{}  /  {}",
            glib::DateTime::now_local()
                .and_then(|t| t.format("%H:%M:%S"))
                .map(|s| s.to_string())
                .unwrap_or_default(),
            if last_update.elapsed().as_secs() > 20 {
                "DATA STALE"
            } else {
                "LIVE"
            }
        ));
        glib::ControlFlow::Continue
    });
    window.present();
}

fn settings_page(prefs: &Rc<RefCell<Preferences>>) -> gtk::Box {
    let p = page(
        "CONSOLE PREFERENCES",
        "Local settings • Changes are stored in your Fedora user profile.",
    );
    let sound = gtk::CheckButton::with_label(
        "Play connection-change sounds (desktop and in-app alerts always enabled)",
    );
    sound.set_active(prefs.borrow().sound);
    sound.set_height_request(56);
    let result = label("", "muted");
    let config = prefs.clone();
    let result_copy = result.clone();
    sound.connect_toggled(move |b| {
        let mut cfg = config.borrow_mut();
        cfg.sound = b.is_active();
        result_copy.set_text(&match cfg.save() {
            Ok(()) => "Saved".into(),
            Err(e) => format!("Could not save: {e}"),
        });
    });
    p.append(&sound);
    p.append(&label("Animations are disabled in this prototype. F11 toggles fullscreen; Escape always leaves fullscreen. Exit returns you to Fedora.", "strip"));
    p.append(&result);
    p.append(&label("DEVELOPMENT BASELINE", "eyebrow"));
    p.append(&label("Rust 1.98.1 • GTK 4.10 / gtk4-rs 0.9\nFedora 38 is end-of-life. Migrate to a supported Fedora release before deployment.\nThis is a desktop application; GNOME continues to manage windows and your session.", "muted"));
    p.append(&label("VALIDATION STILL REQUIRED", "eyebrow"));
    p.append(&label("Physical touchscreen and on-screen keyboard • real PC sensors • x86-64 build • audio alert on actual hardware • desktop-session integration", "muted"));
    p
}

fn network_page() -> (gtk::Box, gtk::Label) {
    let p = page(
        "NETWORK OPERATIONS",
        "Connection awareness and local diagnostics",
    );
    let status = label("CHECKING", "strip");
    p.append(&status);
    let controls = row(12);
    let diagnose = button("RUN LOCAL DIAGNOSTICS", "primary");
    let settings = button("FEDORA NETWORK SETTINGS", "secondary");
    controls.append(&diagnose);
    controls.append(&settings);
    p.append(&controls);
    let output = gtk::TextView::new();
    output.set_editable(false);
    output.set_monospace(true);
    output.set_wrap_mode(gtk::WrapMode::WordChar);
    output.buffer().set_text("Run diagnostics to inspect adapters, addresses, routes, and DNS configuration.\nNo connection will be changed.");
    p.append(&label(
        "ADAPTERS / ROUTES / NAME RESOLUTION",
        "instrument-title",
    ));
    let report_scroll = scrolled(&output);
    report_scroll.set_min_content_height(230);
    p.append(&report_scroll);
    diagnose.connect_clicked(move |b| {
        b.set_sensitive(false);
        output.buffer().set_text("Collecting diagnostics…");
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let _ = tx.send(services::diagnostics());
        });
        let out = output.clone();
        let b = b.clone();
        glib::timeout_add_local(Duration::from_millis(150), move || match rx.try_recv() {
            Ok(text) => {
                out.buffer().set_text(&text);
                b.set_sensitive(true);
                glib::ControlFlow::Break
            }
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                out.buffer()
                    .set_text("Diagnostics worker stopped unexpectedly.");
                b.set_sensitive(true);
                glib::ControlFlow::Break
            }
            Err(_) => glib::ControlFlow::Continue,
        });
    });
    let launch_message = label("", "muted");
    let message = launch_message.clone();
    settings.connect_clicked(move |_| {
        match std::process::Command::new("gnome-control-center")
            .arg("network")
            .spawn()
        {
            Ok(mut child) => {
                std::thread::spawn(move || {
                    let _ = child.wait();
                });
                message.set_text("Network settings opened as a Fedora window.");
            }
            Err(e) => message.set_text(&format!("Could not open settings: {e}")),
        }
    });
    p.append(&launch_message);
    p.append(&crate::tools::speed_panel());
    (p, status)
}

fn applications_page() -> gtk::Box {
    let p = page(
        "APPLICATION DIRECTORY",
        "Native Fedora applications • Launches in standard desktop windows",
    );
    let search = gtk::SearchEntry::new();
    search.set_placeholder_text(Some("Search applications"));
    search.set_height_request(48);
    p.append(&search);
    let message = label("Select an application to launch it.", "muted");
    p.append(&message);
    let list = gtk::ListBox::new();
    list.set_selection_mode(gtk::SelectionMode::None);
    list.add_css_class("directory");
    let mut apps = gio::AppInfo::all();
    apps.retain(|a| a.should_show());
    apps.sort_by_key(|a| a.display_name().to_lowercase());
    let rows: Rc<Vec<(String, gtk::ListBoxRow)>> = Rc::new(
        apps.into_iter()
            .map(|a| {
                let name = a.display_name().to_string();
                let b = button(&format!("▰   {name}"), "app-tile");
                b.set_tooltip_text(a.description().as_deref());
                let msg = message.clone();
                b.connect_clicked(
                    move |_| match a.launch(&[], None::<&gio::AppLaunchContext>) {
                        Ok(()) => msg.set_text(
                            "Application launched. Use the Fedora window switcher to return.",
                        ),
                        Err(e) => msg.set_text(&format!("Launch failed: {e}")),
                    },
                );
                let r = gtk::ListBoxRow::new();
                r.set_child(Some(&b));
                list.append(&r);
                (name.to_lowercase(), r)
            })
            .collect(),
    );
    search.connect_search_changed(move |s| {
        let query = s.text().to_lowercase();
        for (name, r) in rows.iter() {
            r.set_visible(name.contains(&query));
        }
    });
    p.append(&list);
    p
}

#[derive(Clone)]
struct FileItem {
    path: PathBuf,
    name: String,
    directory: bool,
    details: String,
    size: u64,
}

fn load_directory(
    path: PathBuf,
    list: gtk::ListBox,
    entries: Rc<RefCell<Vec<FileItem>>>,
    status: gtk::Label,
    generation: Rc<Cell<u64>>,
) {
    let request = generation.get().wrapping_add(1);
    generation.set(request);
    while let Some(child) = list.first_child() {
        list.remove(&child);
    }
    entries.borrow_mut().clear();
    status.set_text("Reading directory…");
    list.set_sensitive(false);
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let result = (|| -> Result<Vec<FileItem>, String> {
            let mut items = Vec::new();
            for item in std::fs::read_dir(&path).map_err(|e| e.to_string())? {
                let item = item.map_err(|e| e.to_string())?;
                let meta = std::fs::symlink_metadata(item.path()).map_err(|e| e.to_string())?;
                let directory = item.path().is_dir();
                use std::os::unix::fs::PermissionsExt;
                let modified = meta
                    .modified()
                    .ok()
                    .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                    .map(|s| s.as_secs())
                    .unwrap_or(0);
                let modified = glib::DateTime::from_unix_local(modified as i64)
                    .and_then(|d| d.format("%Y-%m-%d %H:%M:%S"))
                    .map(|s| s.to_string())
                    .unwrap_or_else(|_| "Unavailable".into());
                let kind = if meta.file_type().is_symlink() {
                    "Symbolic link"
                } else if directory {
                    "Folder"
                } else {
                    "File"
                };
                items.push(FileItem {
                    path: item.path(),
                    name: item.file_name().to_string_lossy().to_string(),
                    directory,
                    size: meta.len(),
                    details: format!(
                        "{kind}\n{} bytes\nPermissions {:o}\nModified: {modified}\n{}",
                        meta.len(),
                        meta.permissions().mode() & 0o777,
                        item.path().display()
                    ),
                });
                if items.len() >= 2000 {
                    break;
                }
            }
            items.sort_by(|a, b| {
                b.directory
                    .cmp(&a.directory)
                    .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
            });
            Ok(items)
        })();
        let _ = tx.send(result);
    });
    glib::timeout_add_local(Duration::from_millis(100), move || {
        if generation.get() != request {
            return glib::ControlFlow::Break;
        }
        match rx.try_recv() {
            Ok(result) => {
                list.set_sensitive(true);
                match result {
                    Ok(items) => {
                        for item in &items {
                            let r = row(12);
                            r.add_css_class("file-row");
                            r.set_height_request(44);
                            let icon = gtk::DrawingArea::new();
                            icon.set_content_width(30);
                            icon.set_content_height(24);
                            let directory = item.directory;
                            icon.set_draw_func(move |_, c, _, _| {
                                c.set_source_rgb(1.0, 0.73, 0.25);
                                c.rectangle(2.0, 7.0, 25.0, 14.0);
                                if directory {
                                    c.rectangle(2.0, 3.0, 11.0, 5.0);
                                }
                                let _ = c.fill();
                                c.set_source_rgb(0.0, 0.0, 0.0);
                                c.rectangle(7.0, 11.0, 17.0, 2.0);
                                c.rectangle(7.0, 16.0, 12.0, 2.0);
                                let _ = c.fill();
                            });
                            r.append(&icon);
                            let name = label(&item.name, "file-name");
                            name.set_hexpand(true);
                            name.set_wrap(false);
                            name.set_ellipsize(gtk::pango::EllipsizeMode::Middle);
                            r.append(&name);
                            let size = label(
                                &if item.directory {
                                    "DIRECTORY".into()
                                } else {
                                    format!("{} B", item.size)
                                },
                                "muted",
                            );
                            size.set_width_request(90);
                            r.append(&size);
                            list.append(&r);
                        }
                        status.set_text(&format!(
                            "{} entries{} • select to view metadata",
                            items.len(),
                            if items.len() == 2000 {
                                " (display limited to 2,000)"
                            } else {
                                ""
                            }
                        ));
                        *entries.borrow_mut() = items;
                    }
                    Err(e) => status.set_text(&format!("Could not read folder: {e}")),
                }
                glib::ControlFlow::Break
            }
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                list.set_sensitive(true);
                status.set_text("Directory worker stopped unexpectedly.");
                glib::ControlFlow::Break
            }
            Err(_) => glib::ControlFlow::Continue,
        }
    });
}

type PreviewUpdate = Rc<dyn Fn(Option<PathBuf>)>;
fn media_preview(window: &gtk::ApplicationWindow) -> (gtk::Box, PreviewUpdate) {
    let panel = column(8);
    panel.add_css_class("preview-panel");
    panel.append(&label("MEDIA PREVIEW", "instrument-title"));
    let picture = gtk::Picture::new();
    picture.set_can_shrink(true);
    picture.set_content_fit(gtk::ContentFit::Contain);
    picture.set_height_request(180);
    picture.set_visible(false);
    let video = gtk::Video::new();
    video.set_autoplay(false);
    video.set_height_request(180);
    video.set_visible(false);
    video.connect_unmap(|v| {
        if let Some(s) = v.media_stream() {
            s.pause();
        }
    });
    let info = label("Select an image or video.", "muted");
    let view = button("EXPAND PREVIEW", "secondary");
    view.set_sensitive(false);
    panel.append(&picture);
    panel.append(&video);
    panel.append(&info);
    panel.append(&view);
    let current = Rc::new(RefCell::new(None::<(PathBuf, bool)>));
    let generation = Rc::new(Cell::new(0u64));
    let weak = window.downgrade();
    let selected = current.clone();
    let thumbnail = picture.clone();
    let inline_video = video.clone();
    view.connect_clicked(move |_| {
        let Some(parent) = weak.upgrade() else { return };
        let Some((path, is_video)) = selected.borrow().clone() else {
            return;
        };
        if let Some(s) = inline_video.media_stream() {
            s.pause();
        }
        let viewer = gtk::Window::builder()
            .transient_for(&parent)
            .title("LCARS / MEDIA VIEWER")
            .default_width(720)
            .default_height(540)
            .build();
        let body = column(8);
        body.set_margin_start(12);
        body.set_margin_end(12);
        body.set_margin_top(12);
        body.set_margin_bottom(12);
        body.append(&label(
            &path.file_name().unwrap_or_default().to_string_lossy(),
            "page-title",
        ));
        if is_video {
            let player = gtk::Video::for_filename(Some(&path));
            player.set_autoplay(true);
            player.set_hexpand(true);
            player.set_vexpand(true);
            player.connect_unmap(|v| {
                if let Some(s) = v.media_stream() {
                    s.pause();
                }
            });
            body.append(&player);
        } else {
            let full = gtk::Picture::for_paintable(&thumbnail.paintable().expect("Loaded preview"));
            full.set_content_fit(gtk::ContentFit::Contain);
            full.set_can_shrink(true);
            full.set_hexpand(true);
            full.set_vexpand(true);
            body.append(&full);
        }
        let close = button("CLOSE PREVIEW", "primary");
        let weak_viewer = viewer.downgrade();
        close.connect_clicked(move |_| {
            if let Some(w) = weak_viewer.upgrade() {
                w.close();
            }
        });
        body.append(&close);
        viewer.set_child(Some(&body));
        viewer.present();
    });
    let click = gtk::GestureClick::new();
    let view_button = view.clone();
    click.connect_released(move |_, _, _, _| {
        view_button.emit_clicked();
    });
    picture.add_controller(click);
    let update = Rc::new(move |path: Option<PathBuf>| {
        let request = generation.get().wrapping_add(1);
        generation.set(request);
        picture.set_paintable(None::<&gtk::gdk::Paintable>);
        picture.set_visible(false);
        if let Some(s) = video.media_stream() {
            s.pause();
        }
        video.set_file(None::<&gio::File>);
        video.set_visible(false);
        view.set_sensitive(false);
        *current.borrow_mut() = None;
        info.set_text("Select an image or video.");
        let Some(path) = path else { return };
        let (content_type, _) = gio::content_type_guess(Some(&path), &[]);
        let mime = gio::content_type_get_mime_type(&content_type).unwrap_or_default();
        if mime.starts_with("video/") {
            video.set_file(Some(&gio::File::for_path(&path)));
            video.set_visible(true);
            info.set_text("VIDEO / preparing preview…");
            *current.borrow_mut() = Some((path, true));
            if let Some(stream) = video.media_stream() {
                let info = info.clone();
                let view = view.clone();
                let generation = generation.clone();
                let started = std::time::Instant::now();
                glib::timeout_add_local(Duration::from_millis(200), move || {
                    if generation.get() != request {
                        return glib::ControlFlow::Break;
                    }
                    if let Some(error) = stream.error() {
                        info.set_text(&format!(
                            "Preview unavailable: {error}. Use OPEN for an external player."
                        ));
                        return glib::ControlFlow::Break;
                    }
                    if started.elapsed() > Duration::from_secs(20) {
                        info.set_text(
                            "Video preparation timed out. Use OPEN for an external player.",
                        );
                        return glib::ControlFlow::Break;
                    }
                    if stream.is_prepared() {
                        info.set_text(&format!("VIDEO / {} × {} / {:.1} s\nPress Play on the image, or expand preview.", stream.intrinsic_width(), stream.intrinsic_height(), stream.duration() as f64 / 1_000_000.0));
                        view.set_sensitive(true);
                        return glib::ControlFlow::Break;
                    }
                    glib::ControlFlow::Continue
                });
            }
        } else if mime.starts_with("image/") {
            info.set_text("IMAGE / loading preview…");
            let (tx, rx) = std::sync::mpsc::channel();
            let source = path.clone();
            std::thread::spawn(move || {
                let result = (|| -> Result<(Vec<u8>, i32, i32), String> {
                    let size = std::fs::metadata(&source).map_err(|e| e.to_string())?.len();
                    if size > 32 * 1024 * 1024 {
                        return Err("Image exceeds the 32 MB preview limit; use OPEN".into());
                    }
                    let (_, width, height) = gtk::gdk_pixbuf::Pixbuf::file_info(&source)
                        .ok_or("Unsupported image format")?;
                    if i64::from(width) * i64::from(height) > 80_000_000 {
                        return Err("Image exceeds the 80-megapixel preview limit; use OPEN".into());
                    }
                    let pixbuf =
                        gtk::gdk_pixbuf::Pixbuf::from_file_at_scale(&source, 1280, 900, true)
                            .map_err(|e| e.to_string())?;
                    Ok((
                        pixbuf
                            .save_to_bufferv("png", &[])
                            .map_err(|e| e.to_string())?,
                        width,
                        height,
                    ))
                })();
                let _ = tx.send(result);
            });
            let picture = picture.clone();
            let info = info.clone();
            let view = view.clone();
            let current = current.clone();
            let generation = generation.clone();
            glib::timeout_add_local(Duration::from_millis(100), move || {
                if generation.get() != request {
                    return glib::ControlFlow::Break;
                }
                match rx.try_recv() {
                    Ok(Ok((bytes, w, h))) => {
                        match gtk::gdk::Texture::from_bytes(&glib::Bytes::from_owned(bytes)) {
                            Ok(texture) => {
                                picture.set_paintable(Some(&texture));
                                picture.set_visible(true);
                                info.set_text(&format!("IMAGE / {w} × {h} pixels\nTap image to expand. Preview fits without cropping."));
                                *current.borrow_mut() = Some((path.clone(), false));
                                view.set_sensitive(true);
                            }
                            Err(e) => info.set_text(&format!("Preview unavailable: {e}")),
                        }
                        glib::ControlFlow::Break
                    }
                    Ok(Err(e)) => {
                        info.set_text(&format!("Preview unavailable: {e}"));
                        glib::ControlFlow::Break
                    }
                    Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                        info.set_text("Preview worker stopped.");
                        glib::ControlFlow::Break
                    }
                    Err(_) => glib::ControlFlow::Continue,
                }
            });
        } else {
            info.set_text("No media preview for this item. Properties are shown below.");
        }
    });
    (panel, update)
}

fn files_page(window: &gtk::ApplicationWindow) -> gtk::Box {
    let p = page(
        "FILE ACCESS",
        "Browse Fedora files • Linux paths are shown explicitly",
    );
    let controls = row(10);
    let home = button("HOME", "secondary");
    let up = button("UP", "secondary");
    let path = gtk::Entry::new();
    path.set_hexpand(true);
    path.set_text(&services::home().to_string_lossy());
    let go = button("GO / REFRESH", "primary");
    controls.append(&home);
    controls.append(&up);
    controls.append(&path);
    controls.append(&go);
    p.append(&controls);
    let volumes = gio::VolumeMonitor::get();
    let drives = row(8);
    let root = button("C:  SYSTEM /", "secondary");
    drives.append(&root);
    p.append(&drives);
    let mounted = row(8);
    p.append(&mounted);
    let status = label("", "muted");
    p.append(&status);
    let content = row(16);
    content.set_vexpand(true);
    let list = gtk::ListBox::new();
    list.add_css_class("directory");
    list.set_selection_mode(gtk::SelectionMode::Single);
    let list_scroll = scrolled(&list);
    list_scroll.set_min_content_height(250);
    let file_table = column(0);
    file_table.set_hexpand(true);
    let headings = row(12);
    headings.add_css_class("file-headings");
    let name_heading = label("NAME", "table-heading");
    name_heading.set_hexpand(true);
    headings.append(&name_heading);
    let size_heading = label("SIZE / TYPE", "table-heading");
    size_heading.set_width_request(90);
    headings.append(&size_heading);
    file_table.append(&headings);
    file_table.append(&list_scroll);
    content.append(&file_table);
    let detail = column(12);
    detail.set_width_request(260);
    detail.add_css_class("card");
    let (preview, update_preview) = media_preview(window);
    detail.append(&preview);
    detail.append(&label("SELECTION DETAILS", "instrument-title"));
    let metadata = label("Select a file or folder.", "muted");
    metadata.set_selectable(true);
    metadata.set_wrap_mode(gtk::pango::WrapMode::WordChar);
    detail.append(&metadata);
    let open = button("OPEN", "primary");
    let trash = button("MOVE TO TRASH", "danger");
    open.set_sensitive(false);
    trash.set_sensitive(false);
    detail.append(&open);
    detail.append(&trash);
    content.append(&detail);
    p.append(&content);
    let weak_content = content.downgrade();
    let weak_page = p.downgrade();
    p.add_tick_callback(move |_, _| {
        let (Some(content), Some(page)) = (weak_content.upgrade(), weak_page.upgrade()) else {
            return glib::ControlFlow::Break;
        };
        let orientation = if page.width() < 700 {
            gtk::Orientation::Vertical
        } else {
            gtk::Orientation::Horizontal
        };
        if content.orientation() != orientation {
            content.set_orientation(orientation);
        }
        glib::ControlFlow::Continue
    });
    let entries: Rc<RefCell<Vec<FileItem>>> = Rc::new(RefCell::new(Vec::new()));
    let filter = gtk::SearchEntry::new();
    filter.set_placeholder_text(Some("FILTER CURRENT DIRECTORY"));
    let list_copy = list.clone();
    let es_filter = entries.clone();
    filter.connect_search_changed(move |search| {
        let query = search.text().to_lowercase();
        for (i, item) in es_filter.borrow().iter().enumerate() {
            if let Some(r) = list_copy.row_at_index(i as i32) {
                r.set_visible(item.name.to_lowercase().contains(&query));
            }
        }
    });
    p.insert_child_after(&filter, Some(&controls));
    let generation = Rc::new(Cell::new(0u64));
    let navigate: Rc<dyn Fn(PathBuf)> = {
        let list = list.clone();
        let entries = entries.clone();
        let status = status.clone();
        let path = path.clone();
        let metadata = metadata.clone();
        let open = open.clone();
        let trash = trash.clone();
        Rc::new(move |target| {
            filter.set_text("");
            path.set_text(&target.to_string_lossy());
            metadata.set_text("Select a file or folder.");
            open.set_sensitive(false);
            trash.set_sensitive(false);
            load_directory(
                target,
                list.clone(),
                entries.clone(),
                status.clone(),
                generation.clone(),
            );
        })
    };
    let update_mounts: Rc<dyn Fn(&gio::VolumeMonitor)> = {
        let mounted = mounted.clone();
        let navigate = navigate.clone();
        let status = status.clone();
        Rc::new(move |monitor| {
            while let Some(c) = mounted.first_child() {
                mounted.remove(&c);
            }
            let mounts = monitor.mounts();
            let ids: Vec<String> = mounts
                .iter()
                .map(|m| {
                    m.uuid()
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| m.root().uri().to_string())
                })
                .collect();
            match services::drive_labels(&ids) {
                Ok(labels) => {
                    for (mount, letter) in mounts.iter().zip(labels) {
                        if let Some(path) = mount.root().path() {
                            let b = button(&format!("{letter} {}", mount.name()), "secondary");
                            b.set_tooltip_text(Some(&path.to_string_lossy()));
                            let n = navigate.clone();
                            b.connect_clicked(move |_| n(path.clone()));
                            mounted.append(&b);
                        }
                    }
                }
                Err(e) => status.set_text(&e),
            }
        })
    };
    update_mounts(&volumes);
    let update = update_mounts.clone();
    volumes.connect_mount_added(move |m, _| update(m));
    volumes.connect_mount_removed(move |m, _| update_mounts(m));
    for (name, target) in [
        ("DOCUMENTS", services::home().join("Documents")),
        ("DOWNLOADS", services::home().join("Downloads")),
        ("EXCHANGE", PathBuf::from("/mnt/macos")),
    ] {
        let b = button(name, "compact");
        let n = navigate.clone();
        b.connect_clicked(move |_| n(target.clone()));
        drives.append(&b);
    }
    let n = navigate.clone();
    home.connect_clicked(move |_| n(services::home()));
    let n = navigate.clone();
    root.connect_clicked(move |_| n(PathBuf::from("/")));
    let n = navigate.clone();
    let pa = path.clone();
    up.connect_clicked(move |_| {
        let current = PathBuf::from(pa.text().as_str());
        n(current
            .parent()
            .unwrap_or(std::path::Path::new("/"))
            .to_path_buf());
    });
    let n = navigate.clone();
    let pa = path.clone();
    go.connect_clicked(move |_| n(PathBuf::from(pa.text().as_str())));
    let n = navigate.clone();
    path.connect_activate(move |p| n(PathBuf::from(p.text().as_str())));
    let es = entries.clone();
    let o = open.clone();
    let t = trash.clone();
    list.connect_row_selected(move |_, r| {
        update_preview(None);
        if let Some(item) = r.and_then(|r| es.borrow().get(r.index() as usize).cloned()) {
            metadata.set_text(&item.details);
            if !item.directory {
                update_preview(Some(item.path.clone()));
            }
            o.set_sensitive(true);
            t.set_sensitive(true);
        } else {
            o.set_sensitive(false);
            t.set_sensitive(false);
        }
    });
    let open_selected: Rc<dyn Fn()> = {
        let entries = entries.clone();
        let list = list.clone();
        let n = navigate.clone();
        let status = status.clone();
        Rc::new(move || {
            if let Some(item) = list
                .selected_row()
                .and_then(|r| entries.borrow().get(r.index() as usize).cloned())
            {
                if item.directory {
                    n(item.path);
                } else {
                    let uri = gio::File::for_path(&item.path).uri();
                    if let Err(e) =
                        gio::AppInfo::launch_default_for_uri(&uri, None::<&gio::AppLaunchContext>)
                    {
                        status.set_text(&format!("Open failed: {e}"));
                    }
                }
            }
        })
    };
    let action = open_selected.clone();
    open.connect_clicked(move |_| action());
    list.connect_row_activated(move |_, _| open_selected());
    let es = entries.clone();
    let ls = list.clone();
    let weak = window.downgrade();
    let st = status.clone();
    let n = navigate.clone();
    let pa = path.clone();
    trash.connect_clicked(move |_| {
        let Some(item) = ls
            .selected_row()
            .and_then(|r| es.borrow().get(r.index() as usize).cloned())
        else {
            return;
        };
        let Some(w) = weak.upgrade() else { return };
        let dialog = gtk::MessageDialog::builder()
            .transient_for(&w)
            .modal(true)
            .message_type(gtk::MessageType::Question)
            .text(format!("Move {} to Trash?", item.name))
            .secondary_text(format!(
                "{}\nNo permanent-delete fallback is used.",
                item.path.display()
            ))
            .build();
        dialog.add_button("Cancel", gtk::ResponseType::Cancel);
        dialog.add_button("Move to Trash", gtk::ResponseType::Accept);
        dialog.set_default_response(gtk::ResponseType::Cancel);
        let st = st.clone();
        let n = n.clone();
        let pa = pa.clone();
        dialog.connect_response(move |d, response| {
            if response == gtk::ResponseType::Accept {
                let status = st.clone();
                let navigate = n.clone();
                let parent = PathBuf::from(pa.text().as_str());
                gio::File::for_path(&item.path).trash_async(
                    glib::Priority::DEFAULT,
                    None::<&gio::Cancellable>,
                    move |result| match result {
                        Ok(()) => navigate(parent),
                        Err(e) => status
                            .set_text(&format!("Trash failed; nothing permanently deleted: {e}")),
                    },
                );
            }
            d.close();
        });
        dialog.present();
    });
    navigate(services::home());
    p
}
