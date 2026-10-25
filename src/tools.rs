// Mixed provenance: Codex tools baseline; Claude AI continuation (user-attributed)
// added structured findings and cancellation-result refinements. See docs/WORK_PROVENANCE.md.
use gtk::{glib, prelude::*};
use lcars_desktop::attention::Task;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc, Arc,
};
use std::time::Duration;

fn label(s: &str) -> gtk::Label {
    let l = gtk::Label::new(Some(s));
    l.set_xalign(0.0);
    l.set_wrap(true);
    l
}
fn button(s: &str) -> gtk::Button {
    let b = gtk::Button::with_label(s);
    b.set_height_request(44);
    b
}

fn section(title: &str) -> gtk::Box {
    let b = gtk::Box::new(gtk::Orientation::Vertical, 4);
    b.add_css_class("card");
    let t = label(title);
    t.add_css_class("instrument-title");
    b.append(&t);
    b
}
fn cell(text: &str, class: &str, width: i32) -> gtk::Label {
    let l = gtk::Label::new(Some(text));
    l.set_xalign(0.0);
    l.add_css_class(class);
    l.set_ellipsize(gtk::pango::EllipsizeMode::End);
    if width > 0 {
        l.set_width_request(width);
    }
    l
}
/// Readable findings replace the terminal dump Step 10 forbids.
fn render_findings(
    into: &gtk::Box,
    kernel: &str,
    storage: &[crate::services::StorageRow],
    procs: &[crate::services::ProcessRow],
) {
    crate::ui::clear_focus_within(into);
    while let Some(c) = into.first_child() {
        into.remove(&c);
    }
    let k = section("KERNEL");
    k.append(&cell(kernel.trim(), "file-row", 0));
    into.append(&k);

    let st = section("STORAGE / MOUNTED VOLUMES");
    for row in storage.iter().take(10) {
        let r = gtk::Box::new(gtk::Orientation::Horizontal, 10);
        r.append(&cell(&row.mount, "file-row", 150));
        r.append(&cell(&row.fstype, "muted", 70));
        r.append(&cell(
            &format!("{} / {} free", row.size, row.avail),
            "muted",
            150,
        ));
        let bar = gtk::ProgressBar::new();
        bar.set_fraction(f64::from(row.use_pct) / 100.0);
        bar.set_hexpand(true);
        bar.set_valign(gtk::Align::Center);
        r.append(&bar);
        r.append(&cell(&format!("{}%", row.use_pct), "status", 50));
        st.append(&r);
    }
    if storage.is_empty() {
        st.append(&cell("No volumes reported.", "muted", 0));
    }
    into.append(&st);

    let pr = section("TOP PROCESSES / BY PROCESSOR SHARE");
    let head = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    head.add_css_class("file-headings");
    head.append(&cell("PID", "table-heading", 70));
    head.append(&cell("COMMAND", "table-heading", 200));
    head.append(&cell("CPU", "table-heading", 70));
    head.append(&cell("MEMORY", "table-heading", 70));
    pr.append(&head);
    for row in procs.iter().take(12) {
        let r = gtk::Box::new(gtk::Orientation::Horizontal, 10);
        r.append(&cell(&row.pid.to_string(), "muted", 70));
        r.append(&cell(&row.command, "file-row", 200));
        r.append(&cell(&format!("{:.1}%", row.cpu), "status", 70));
        r.append(&cell(&format!("{:.1}%", row.mem), "muted", 70));
        pr.append(&r);
    }
    if procs.is_empty() {
        pr.append(&cell("No process data available.", "muted", 0));
    }
    into.append(&pr);
}
pub fn speed_panel(demand: crate::ui::TaskDemand) -> (gtk::Box, gtk::Label) {
    let p = gtk::Box::new(gtk::Orientation::Vertical, 10);
    p.add_css_class("card");
    p.append(&label("SUBSPACE TRANSFER / SPEED TEST"));
    p.append(&label("Cloudflare HTTPS endpoint • downloads 10 MB + uploads 2 MB of generated data. Your IP is visible to Cloudflare. No user files are sent. Single-connection sample; latency includes TLS/HTTP setup."));
    let controls = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    let start = button("START 12 MB TEST");
    let stop = button("CANCEL");
    stop.set_sensitive(false);
    controls.append(&start);
    controls.append(&stop);
    p.append(&controls);
    crate::ui::adaptive_row(&controls, p.upcast_ref(), 380);
    let result = label("READY / no test has run");
    result.set_selectable(true);
    p.append(&result);
    let cancel = Arc::new(AtomicBool::new(false));
    let c = cancel.clone();
    stop.connect_clicked(move |_| {
        c.store(true, Ordering::Relaxed);
    });
    let summary = result.clone();
    start.connect_clicked(move |b| {
        demand.borrow_mut().set(Task::Transfer, true);
        b.set_sensitive(false);
        stop.set_sensitive(true);
        cancel.store(false, Ordering::Relaxed);
        let (tx, rx) = mpsc::channel();
        let c = cancel.clone();
        std::thread::spawn(move || {
            let out = crate::network::speed_test(c, tx.clone());
            let _ = tx.send(format!(
                "DONE {}",
                out.unwrap_or_else(|e| format!("TEST FAILED / {e}"))
            ));
        });
        let b = b.clone();
        let stop = stop.clone();
        let result = result.clone();
        let demand = demand.clone();
        let cancelled = cancel.clone();
        glib::timeout_add_local(Duration::from_millis(100), move || {
            while let Ok(message) = rx.try_recv() {
                if let Some(done) = message.strip_prefix("DONE ") {
                    demand.borrow_mut().set(Task::Transfer, false);
                    // A cancelled run must not leave a partial latency reading on
                    // screen, which would read as a completed measurement.
                    if cancelled.load(Ordering::Relaxed) {
                        result.set_text("TEST CANCELLED / no throughput recorded");
                    } else {
                        result.set_text(done);
                    }
                    b.set_sensitive(true);
                    stop.set_sensitive(false);
                    return glib::ControlFlow::Break;
                }
                if !cancelled.load(Ordering::Relaxed) {
                    result.set_text(&message);
                }
            }
            glib::ControlFlow::Continue
        });
    });
    (p, summary)
}

pub fn page(demand: crate::ui::TaskDemand) -> (gtk::Box, gtk::Label, gtk::TextBuffer) {
    let p = gtk::Box::new(gtk::Orientation::Vertical, 14);
    p.set_margin_start(24);
    p.set_margin_end(24);
    p.set_margin_top(20);
    p.set_margin_bottom(20);
    let title = label("COMPUTER UTILITIES / 047");
    title.add_css_class("page-title");
    p.append(&title);
    let calc = gtk::Box::new(gtk::Orientation::Vertical, 8);
    calc.add_css_class("card");
    calc.append(&label("LCARS CALCULATOR / TWO-OPERAND ARITHMETIC"));
    let row = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    let a = gtk::Entry::new();
    a.set_placeholder_text(Some("First number"));
    let b = gtk::Entry::new();
    b.set_placeholder_text(Some("Second number"));
    row.append(&a);
    row.append(&b);
    calc.append(&row);
    crate::ui::adaptive_row(&row, p.upcast_ref(), 520);
    let out = label("0");
    out.add_css_class("metric");
    out.set_selectable(true);
    calc.append(&out);
    let ops = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    for op in ["+", "−", "×", "÷"] {
        let bt = button(op);
        let a = a.clone();
        let b = b.clone();
        let out = out.clone();
        bt.connect_clicked(move |_| {
            out.set_text(&match (a.text().parse::<f64>(), b.text().parse::<f64>()) {
                (Ok(x), Ok(y)) if x.is_finite() && y.is_finite() => match op {
                    "+" => format!("{}", x + y),
                    "−" => format!("{}", x - y),
                    "×" => format!("{}", x * y),
                    _ if y == 0.0 => "Cannot divide by zero".into(),
                    _ => format!("{}", x / y),
                },
                _ => "Enter two finite numbers".into(),
            });
        });
        ops.append(&bt);
    }
    calc.append(&ops);
    p.append(&calc);
    let refresh = button("SCAN SYSTEM / PROCESS DIAGNOSTICS");
    p.append(&refresh);
    let idle = "Read-only scan: kernel, storage, and top processes. No process is terminated.";
    let status = label(idle);
    status.add_css_class("muted");
    p.append(&status);
    let findings = gtk::Box::new(gtk::Orientation::Vertical, 12);
    let scroll = gtk::ScrolledWindow::builder()
        .child(&findings)
        .min_content_height(220)
        .vexpand(true)
        .build();
    p.append(&scroll);
    // The buffer stays the machine-readable record the overview summary reads.
    let output = gtk::TextView::new();
    output.set_editable(false);
    output.buffer().set_text(idle);
    let summary = output.buffer();
    refresh.connect_clicked(move |b| {
        demand.borrow_mut().set(Task::Diagnostics, true);
        b.set_sensitive(false);
        status.set_text("SCANNING / reading kernel, storage and processes…");
        let (tx, rx) = mpsc::channel::<Result<(String, String, String), String>>();
        std::thread::spawn(move || {
            let kernel = crate::services::command("uname", &["-srmo"]);
            let storage = crate::services::command("df", &["-hT"]);
            let procs =
                crate::services::command("ps", &["-eo", "pid,comm,pcpu,pmem", "--sort=-pcpu"]);
            let _ = tx.send(match (kernel, storage, procs) {
                (Ok(k), Ok(s), Ok(p)) => Ok((k, s, p)),
                (Err(e), _, _) | (_, Err(e), _) | (_, _, Err(e)) => Err(e),
            });
        });
        let b = b.clone();
        let output = output.clone();
        let demand = demand.clone();
        let findings = findings.clone();
        let status = status.clone();
        glib::timeout_add_local(Duration::from_millis(150), move || match rx.try_recv() {
            Ok(Ok((kernel, storage_raw, procs_raw))) => {
                demand.borrow_mut().set(Task::Diagnostics, false);
                let storage = crate::services::parse_storage(&storage_raw);
                let procs = crate::services::parse_processes(&procs_raw);
                render_findings(&findings, &kernel, &storage, &procs);
                status.set_text(&format!(
                    "SCAN COMPLETE / {} volumes, {} processes / read-only, nothing terminated",
                    storage.len(),
                    procs.len()
                ));
                output.buffer().set_text(&format!(
                    "KERNEL\n{kernel}\nSTORAGE\n{storage_raw}\nPROCESSES\n{procs_raw}"
                ));
                b.set_sensitive(true);
                glib::ControlFlow::Break
            }
            Ok(Err(e)) => {
                demand.borrow_mut().set(Task::Diagnostics, false);
                status.set_text(&format!("SCAN FAILED / {e}"));
                output.buffer().set_text(&format!("SCAN FAILED / {e}"));
                b.set_sensitive(true);
                glib::ControlFlow::Break
            }
            Err(_) => glib::ControlFlow::Continue,
        });
    });
    (p, out, summary)
}
