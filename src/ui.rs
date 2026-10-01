use crate::services::{self, Preferences};
use gtk::{gio, glib, prelude::*};
use std::cell::{Cell, RefCell};
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
    let p = column(18);
    p.set_margin_top(20);
    p.set_margin_bottom(20);
    p.set_margin_start(24);
    p.set_margin_end(24);
    p.append(&label(title, "page-title"));
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
    c.append(&label(title, "eyebrow"));
    let value = label("—", "metric");
    let bar = gtk::ProgressBar::new();
    c.append(&value);
    c.append(&bar);
    (c, value, bar)
}

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
        .default_width(1080)
        .default_height(700)
        .build();
    let preferences = Rc::new(RefCell::new(Preferences::load()));
    let root = column(0);
    let header = row(12);
    header.add_css_class("masthead");
    let brand = label("LCARS", "brand");
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
    root.append(&header);
    let body = row(0);
    body.set_vexpand(true);
    let rail = column(8);
    rail.add_css_class("rail");
    rail.set_width_request(176);
    rail.append(&label("01–47\nOPERATIONS", "rail-cap"));
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
    hero_text.append(&label("COMPUTER\nOPERATIONS", "hero"));
    hero_text.append(&label("Your Fedora workstation, at a glance.", "muted"));
    hero.append(&hero_text);
    let system_id = label("LCARS\n01 / 2026\nFEDORA NODE", "system-id");
    hero.append(&system_id);
    dashboard.append(&hero);
    let metrics = gtk::Grid::builder()
        .column_spacing(14)
        .row_spacing(14)
        .column_homogeneous(true)
        .build();
    let (cpu_card, cpu_value, cpu_bar) = card("01 / PROCESSOR LOAD");
    let (mem_card, mem_value, mem_bar) = card("02 / MEMORY UTILIZATION");
    let (net_card, net_value, _) = card("03 / NETWORK RECEIVE");
    let (time_card, time_value, _) = card("04 / SYSTEM UPTIME");
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
    stack.add_named(&scrolled(&settings_page(&preferences)), Some("settings"));
    let nav_items = [
        ("01  OVERVIEW", "dashboard"),
        ("02  FILES", "files"),
        ("03  NETWORK", "network"),
        ("04  APPLICATIONS", "apps"),
        ("05  PREFERENCES", "settings"),
    ];
    let nav_buttons: Rc<RefCell<Vec<gtk::Button>>> = Rc::new(RefCell::new(Vec::new()));
    for (text, name) in nav_items {
        let b = button(text, "nav");
        if name == "dashboard" {
            b.add_css_class("selected");
        }
        let st = stack.clone();
        let buttons = nav_buttons.clone();
        b.connect_clicked(move |this| {
            st.set_visible_child_name(name);
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
        "LOCAL SYSTEM\nFEDORA 38\nPROTOTYPE 0.1",
        "rail-footer",
    ));
    body.append(&rail);
    body.append(&stack);
    root.append(&body);
    let footer = row(12);
    footer.add_css_class("footer");
    let indicator = label("● CHECKING NETWORK", "status");
    indicator.set_hexpand(true);
    let clock = label("", "muted");
    footer.append(&indicator);
    footer.append(&clock);
    root.append(&footer);
    window.set_child(Some(&root));
    let keys = gtk::EventControllerKey::new();
    let weak = window.downgrade();
    keys.connect_key_pressed(move |_, key, _, _| {
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
    let rx = services::monitor();
    let mut previous = String::new();
    let mut last_update = std::time::Instant::now();
    let weak = window.downgrade();
    glib::timeout_add_local(Duration::from_millis(250), move || {
        if weak.upgrade().is_none() {
            return glib::ControlFlow::Break;
        }
        while let Ok(s) = rx.try_recv() {
            last_update = std::time::Instant::now();
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
            indicator.set_text(&format!("● {}", s.network));
            if s.network == "OFFLINE" || s.network == "LIMITED" {
                indicator.add_css_class("warning");
            } else {
                indicator.remove_css_class("warning");
            }
            network_status.set_text(&format!("{}\nReceive {:.0} KiB/s   •   Transmit {:.0} KiB/s\nState sampled every 5 seconds; reachability follows NetworkManager's check policy.",s.network,s.received as f64/1024.0,s.transmitted as f64/1024.0));
            if previous == "ONLINE" && s.network == "OFFLINE" && preferences.borrow().sound {
                std::thread::spawn(|| {
                    let _ = services::command("canberra-gtk-play", &["-i", "dialog-warning"]);
                });
            }
            previous = s.network;
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
    let sound =
        gtk::CheckButton::with_label("Play an alert when an online connection goes offline");
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
    p.append(&label("Physical touchscreen and on-screen keyboard • real PC sensors • x86-64 build • audio alert on actual hardware • speed-test provider • desktop-session integration", "muted"));
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
    p.append(&label("SPEED TEST / NOT CONFIGURED", "eyebrow"));
    p.append(&label("Choose a measurement provider and data-transfer limits before enabling tests. Network transfer rates above are live interface traffic, not a speed-test result. Automated repairs and external probes are not enabled in this prototype.", "muted"));
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
                    details: format!(
                        "{kind}\n{} bytes\nPermissions {:o}\nModified Unix time: {modified}\n{}",
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
                            let l = label(
                                &format!(
                                    "{}   {}",
                                    if item.directory { "▰" } else { "▱" },
                                    item.name
                                ),
                                "file-row",
                            );
                            l.set_height_request(48);
                            list.append(&l);
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
    let go = button("GO", "primary");
    controls.append(&home);
    controls.append(&up);
    controls.append(&path);
    controls.append(&go);
    p.append(&controls);
    let volumes = gio::VolumeMonitor::get();
    let drives = row(8);
    let root = button("C:  SYSTEM /", "secondary");
    drives.append(&root);
    let volume_text = label(
        "Other mounts: use their Linux paths. Stable drive-letter assignment is pending.",
        "muted",
    );
    p.append(&volume_text);
    let update_mounts = move |monitor: &gio::VolumeMonitor| {
        let names: Vec<String> = monitor
            .mounts()
            .iter()
            .map(|m| format!("{} → {}", m.name(), m.root().parse_name()))
            .collect();
        volume_text.set_text(&format!(
            "MOUNTED VOLUMES\n{}\nAdditional drive letters are not assigned yet.",
            if names.is_empty() {
                "No additional user volumes".to_string()
            } else {
                names.join("\n")
            }
        ));
    };
    update_mounts(&volumes);
    // A fresh page is built each application launch; hotplug is reported by the monitor below.
    let mount_label = label("", "muted");
    let mount_label2 = mount_label.clone();
    volumes.connect_mount_added(move |_, m| {
        mount_label2.set_text(&format!(
            "Volume connected: {}. Navigate using its Linux path.",
            m.name()
        ))
    });
    let mount_label2 = mount_label.clone();
    volumes.connect_mount_removed(move |_, m| {
        mount_label2.set_text(&format!("Volume removed: {}", m.name()))
    });
    p.append(&drives);
    p.append(&mount_label);
    let status = label("", "muted");
    p.append(&status);
    let content = row(16);
    content.set_vexpand(true);
    let list = gtk::ListBox::new();
    list.add_css_class("directory");
    list.set_selection_mode(gtk::SelectionMode::Single);
    let list_scroll = scrolled(&list);
    list_scroll.set_min_content_height(250);
    content.append(&list_scroll);
    let detail = column(12);
    detail.set_width_request(240);
    detail.add_css_class("card");
    detail.append(&label("SELECTION DETAILS", "eyebrow"));
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
    let entries: Rc<RefCell<Vec<FileItem>>> = Rc::new(RefCell::new(Vec::new()));
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
        if let Some(item) = r.and_then(|r| es.borrow().get(r.index() as usize).cloned()) {
            metadata.set_text(&item.details);
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
