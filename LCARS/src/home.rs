use eframe::egui;
use egui::{Color32, CornerRadius, Pos2, Rect, Vec2};

use crate::LcarsApp;

pub fn show(app: &mut LcarsApp, ctx: &egui::Context) {
    let background = Color32::from_rgb(10, 9, 18);

    let orange = Color32::from_rgb(255, 165, 80);
    let purple = Color32::from_rgb(180, 120, 210);
    let mauve = Color32::from_rgb(190, 130, 180);
    let blue = Color32::from_rgb(100, 170, 220);
    let yellow = Color32::from_rgb(235, 190, 80);

    egui::CentralPanel::default()
        .frame(egui::Frame::NONE.fill(background))
        .show(ctx, |ui| {
            let available = ui.available_rect_before_wrap();

            let sidebar_width = 180.0_f32;

            // -------------------------------------------------
            // SIDEBAR
            // -------------------------------------------------

            ui.painter().rect_filled(
                Rect::from_min_size(
                    available.min,
                    Vec2::new(sidebar_width, available.height()),
                ),
                CornerRadius::ZERO,
                Color32::from_rgb(30, 23, 40),
            );

            // LCARS title

            let title_rect = Rect::from_min_size(
                Pos2::new(0.0_f32, 0.0_f32),
                Vec2::new(sidebar_width, 70.0_f32),
            );

            ui.painter().rect_filled(
                title_rect,
                CornerRadius {
                    nw: 0,
                    ne: 0,
                    sw: 0,
                    se: 25,
                },
                orange,
            );

            ui.painter().text(
                Pos2::new(25.0_f32, 35.0_f32),
                egui::Align2::LEFT_CENTER,
                "LCARS",
                egui::FontId::proportional(28.0_f32),
                Color32::BLACK,
            );

            // Sidebar buttons

            let buttons = [
                ("SYSTEM", purple),
                ("NETWORK", blue),
                ("FILES", mauve),
                ("STATUS", yellow),
            ];

            let mut y = 95.0_f32;

            for (label, color) in buttons {
                let rect = Rect::from_min_size(
                    Pos2::new(15.0_f32, y),
                    Vec2::new(150.0_f32, 50.0_f32),
                );

                let response = ui.allocate_rect(
                    rect,
                    egui::Sense::click(),
                );

                let display_color = if response.hovered() {
                    color.gamma_multiply(1.2_f32)
                } else {
                    color
                };

                ui.painter().rect_filled(
                    rect,
                    CornerRadius {
                        nw: 18,
                        ne: 0,
                        sw: 18,
                        se: 0,
                    },
                    display_color,
                );

                ui.painter().text(
                    rect.center(),
                    egui::Align2::CENTER_CENTER,
                    label,
                    egui::FontId::proportional(15.0_f32),
                    Color32::BLACK,
                );

                y += 65.0_f32;
            }

            // Logout button

            let logout_rect = Rect::from_min_size(
                Pos2::new(15.0_f32, available.bottom() - 75.0_f32),
                Vec2::new(150.0_f32, 45.0_f32),
            );

            let logout_response =
                ui.allocate_rect(logout_rect, egui::Sense::click());

            let logout_color = if logout_response.hovered() {
                orange.gamma_multiply(1.2_f32)
            } else {
                orange
            };

            ui.painter().rect_filled(
                logout_rect,
                CornerRadius {
                    nw: 18,
                    ne: 0,
                    sw: 18,
                    se: 0,
                },
                logout_color,
            );

            ui.painter().text(
                logout_rect.center(),
                egui::Align2::CENTER_CENTER,
                "LOGOUT",
                egui::FontId::proportional(14.0_f32),
                Color32::BLACK,
            );

            if logout_response.clicked() {
                app.username.clear();
                app.password.clear();
                app.login_message =
                    "AWAITING USER AUTHENTICATION".to_string();

                app.screen = crate::Screen::Login;
            }

            // -------------------------------------------------
            // MAIN HEADER
            // -------------------------------------------------

            let main_left = sidebar_width + 25.0_f32;

            let header = Rect::from_min_size(
                Pos2::new(main_left, 15.0_f32),
                Vec2::new(
                    available.width() - main_left - 20.0_f32,
                    60.0_f32,
                ),
            );

            ui.painter().rect_filled(
                header,
                CornerRadius {
                    nw: 25,
                    ne: 0,
                    sw: 0,
                    se: 25,
                },
                mauve,
            );

            ui.painter().text(
                Pos2::new(main_left + 25.0_f32, 45.0_f32),
                egui::Align2::LEFT_CENTER,
                "MAIN SYSTEM INTERFACE",
                egui::FontId::proportional(25.0_f32),
                Color32::BLACK,
            );

            // -------------------------------------------------
            // WELCOME MESSAGE
            // -------------------------------------------------

            ui.painter().text(
                Pos2::new(main_left + 30.0_f32, 115.0_f32),
                egui::Align2::LEFT_CENTER,
                format!("WELCOME, {}", app.username.to_uppercase()),
                egui::FontId::proportional(22.0_f32),
                orange,
            );

            // -------------------------------------------------
            // SYSTEM STATUS PANELS
            // -------------------------------------------------

            let panel_y = 150.0_f32;
            let panel_width = 190.0_f32;
            let panel_height = 120.0_f32;

            let panels = [
                (
                    "CPU",
                    "ONLINE",
                    purple,
                ),
                (
                    "MEMORY",
                    "ONLINE",
                    blue,
                ),
                (
                    "STORAGE",
                    "ONLINE",
                    mauve,
                ),
                (
                    "NETWORK",
                    "ONLINE",
                    orange,
                ),
            ];

            for (i, (title, status, color)) in panels.iter().enumerate() {
                let x = main_left
                    + 30.0_f32
                    + i as f32 * (panel_width + 15.0_f32);

                let rect = Rect::from_min_size(
                    Pos2::new(x, panel_y),
                    Vec2::new(panel_width, panel_height),
                );

                ui.painter().rect_filled(
                    rect,
                    CornerRadius {
                        nw: 20,
                        ne: 0,
                        sw: 20,
                        se: 0,
                    },
                    Color32::from_rgb(30, 25, 45),
                );

                // Colored top bar

                let bar = Rect::from_min_size(
                    rect.min,
                    Vec2::new(panel_width, 12.0_f32),
                );

                ui.painter().rect_filled(
                    bar,
                    CornerRadius {
                        nw: 20,
                        ne: 0,
                        sw: 0,
                        se: 0,
                    },
                    *color,
                );

                ui.painter().text(
                    Pos2::new(
                        rect.left() + 20.0_f32,
                        rect.top() + 45.0_f32,
                    ),
                    egui::Align2::LEFT_CENTER,
                    *title,
                    egui::FontId::proportional(17.0_f32),
                    *color,
                );

                ui.painter().text(
                    Pos2::new(
                        rect.left() + 20.0_f32,
                        rect.top() + 78.0_f32,
                    ),
                    egui::Align2::LEFT_CENTER,
                    *status,
                    egui::FontId::proportional(13.0_f32),
                    Color32::LIGHT_GREEN,
                );
            }

            // -------------------------------------------------
            // LOWER INFORMATION PANEL
            // -------------------------------------------------

            let info_rect = Rect::from_min_size(
                Pos2::new(main_left + 30.0_f32, 310.0_f32),
                Vec2::new(
                    available.width() - main_left - 60.0_f32,
                    190.0_f32,
                ),
            );

            ui.painter().rect_filled(
                info_rect,
                CornerRadius {
                    nw: 25,
                    ne: 0,
                    sw: 25,
                    se: 0,
                },
                Color32::from_rgb(25, 20, 35),
            );

            ui.painter().text(
                Pos2::new(
                    info_rect.left() + 25.0_f32,
                    info_rect.top() + 35.0_f32,
                ),
                egui::Align2::LEFT_CENTER,
                "SYSTEM INFORMATION",
                egui::FontId::proportional(20.0_f32),
                purple,
            );

            ui.painter().text(
                Pos2::new(
                    info_rect.left() + 25.0_f32,
                    info_rect.top() + 75.0_f32,
                ),
                egui::Align2::LEFT_CENTER,
                "LCARS INTERFACE",
                egui::FontId::proportional(14.0_f32),
                Color32::WHITE,
            );

            ui.painter().text(
                Pos2::new(
                    info_rect.left() + 25.0_f32,
                    info_rect.top() + 105.0_f32,
                ),
                egui::Align2::LEFT_CENTER,
                "SYSTEM STATUS: NOMINAL",
                egui::FontId::proportional(14.0_f32),
                Color32::LIGHT_GREEN,
            );

            ui.painter().text(
                Pos2::new(
                    info_rect.left() + 25.0_f32,
                    info_rect.top() + 135.0_f32,
                ),
                egui::Align2::LEFT_CENTER,
                "USER AUTHENTICATED",
                egui::FontId::proportional(14.0_f32),
                blue,
            );

            // -------------------------------------------------
            // BOTTOM BAR
            // -------------------------------------------------

            let bottom = Rect::from_min_size(
                Pos2::new(main_left, available.bottom() - 50.0_f32),
                Vec2::new(
                    available.width() - main_left - 20.0_f32,
                    38.0_f32,
                ),
            );

            ui.painter().rect_filled(
                bottom,
                CornerRadius {
                    nw: 0,
                    ne: 20,
                    sw: 20,
                    se: 0,
                },
                purple,
            );

            ui.painter().text(
                bottom.center(),
                egui::Align2::CENTER_CENTER,
                "SYSTEM ONLINE",
                egui::FontId::proportional(14.0_f32),
                Color32::BLACK,
            );
        });
}
