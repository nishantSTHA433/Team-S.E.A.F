mod network;
mod services;
mod tools;
mod ui;

use gtk::prelude::*;

fn main() -> gtk::glib::ExitCode {
    if std::env::args().any(|s| s == "--speed-check") {
        let (tx, _rx) = std::sync::mpsc::channel();
        return match network::speed_test(
            std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
            tx,
        ) {
            Ok(result) => {
                println!("{result}");
                gtk::glib::ExitCode::SUCCESS
            }
            Err(error) => {
                eprintln!("{error}");
                gtk::glib::ExitCode::FAILURE
            }
        };
    }
    // CPU rendering avoids the virtual GPU corruption seen in UTM. Override explicitly to opt out.
    if std::env::var_os("GSK_RENDERER").is_none() {
        std::env::set_var("GSK_RENDERER", "cairo");
    }
    let app = gtk::Application::builder()
        .application_id("org.lcars.Console")
        .build();
    app.connect_activate(ui::build);
    app.run()
}
