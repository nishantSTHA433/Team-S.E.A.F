use gtk::{glib, prelude::*};
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

pub fn speed_panel() -> gtk::Box {
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
    let result = label("READY / no test has run");
    result.set_selectable(true);
    p.append(&result);
    let cancel = Arc::new(AtomicBool::new(false));
    let c = cancel.clone();
    stop.connect_clicked(move |_| {
        c.store(true, Ordering::Relaxed);
    });
    start.connect_clicked(move |b| {
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
        glib::timeout_add_local(Duration::from_millis(100), move || {
            while let Ok(message) = rx.try_recv() {
                if let Some(done) = message.strip_prefix("DONE ") {
                    result.set_text(done);
                    b.set_sensitive(true);
                    stop.set_sensitive(false);
                    return glib::ControlFlow::Break;
                }
                result.set_text(&message);
            }
            glib::ControlFlow::Continue
        });
    });
    p
}

pub fn page() -> gtk::Box {
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
    let output = gtk::TextView::new();
    output.set_editable(false);
    output.set_monospace(true);
    output
        .buffer()
        .set_text("Read-only scan: kernel, storage, and top processes. No process is terminated.");
    let scroll = gtk::ScrolledWindow::builder()
        .child(&output)
        .min_content_height(240)
        .vexpand(true)
        .build();
    p.append(&scroll);
    refresh.connect_clicked(move |b| {
        b.set_sensitive(false);
        let (tx, rx) = mpsc::channel();
        std::thread::spawn(move || {
            let mut report = String::new();
            for (name, cmd, args) in [
                ("KERNEL", "uname", vec!["-srmo"]),
                ("STORAGE", "df", vec!["-hT"]),
                (
                    "PROCESSES / lifetime average CPU",
                    "ps",
                    vec!["-eo", "pid,comm,pcpu,pmem", "--sort=-pcpu"],
                ),
            ] {
                report.push_str(&format!(
                    "{name}\n{}\n\n",
                    crate::services::command(cmd, &args)
                        .unwrap_or_else(|e| e)
                        .lines()
                        .take(24)
                        .collect::<Vec<_>>()
                        .join("\n")
                ));
            }
            let _ = tx.send(report);
        });
        let b = b.clone();
        let output = output.clone();
        glib::timeout_add_local(Duration::from_millis(150), move || {
            if let Ok(s) = rx.try_recv() {
                output.buffer().set_text(&s);
                b.set_sensitive(true);
                glib::ControlFlow::Break
            } else {
                glib::ControlFlow::Continue
            }
        });
    });
    p
}
