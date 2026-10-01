mod services;
mod ui;

use gtk::prelude::*;

fn main() -> gtk::glib::ExitCode {
    let app = gtk::Application::builder()
        .application_id("org.lcars.Console")
        .build();
    app.connect_activate(ui::build);
    app.run()
}
