mod home;
mod login;

use eframe::egui;

#[derive(PartialEq)]
pub enum Screen {
    Login,
    Home,
}

pub struct LcarsApp {
    pub screen: Screen,
    pub username: String,
    pub password: String,
    pub login_message: String,
}

impl Default for LcarsApp {
    fn default() -> Self {
        Self {
            screen: Screen::Login,
            username: String::new(),
            password: String::new(),
            login_message: String::from("AWAITING USER AUTHENTICATION"),
        }
    }
}

impl eframe::App for LcarsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match self.screen {
            Screen::Login => {
                login::show(self, ctx);
            }

            Screen::Home => {
                home::show(self, ctx);
            }
        }

        ctx.request_repaint();
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "LCARS System Interface",
        options,
        Box::new(|_cc| Ok(Box::new(LcarsApp::default()))),
    )
}
