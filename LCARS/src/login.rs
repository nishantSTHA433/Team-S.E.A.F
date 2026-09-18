use eframe::egui;
use egui::{Color32, CornerRadius, Pos2, Rect, Stroke, Vec2};

use crate::{LcarsApp, Screen};

pub fn show(app: &mut LcarsApp, ctx: &egui::Context) {
    let background = Color32::from_rgb(12, 10, 20);
    let orange = Color32::from_rgb(255, 165, 80);
    let purple = Color32::from_rgb(180, 120, 210);
    let mauve = Color32::from_rgb(190, 130, 180);
    let blue = Color32::from_rgb(100, 170, 220);
    let dark_panel = Color32::from_rgb(25, 20, 35);
    let white = Color32::from_rgb(235, 235, 240);

    egui::CentralPanel::default()
        .frame(egui::Frame::NONE.fill(background))
        .show(ctx, |ui| {
            let available = ui.available_rect_before_wrap();

            // -------------------------------------------------
            // LEFT SIDEBAR
            // -------------------------------------------------

            let sidebar_width = 170.0_f32;

            let sidebar_rect = Rect::from_min_size(
                available.min,
                Vec2::new(sidebar_width, available.height()),
            );

            ui.painter().rect_filled(
                sidebar_rect,
                CornerRadius::ZERO,
                Color32::from_rgb(35, 25, 45),
            );

            // LCARS header

            let top_bar = Rect::from_min_size(
                Pos2::new(0.0_f32, 0.0_f32),
                Vec2::new(sidebar_width, 65.0_f32),
            );

            ui.painter().rect_filled(
                top_bar,
                CornerRadius {
                    nw: 0,
                    ne: 0,
                    sw: 0,
                    se: 25,
                },
                orange,
            );

            ui.painter().text(
                Pos2::new(20.0_f32, 32.0_f32),
                egui::Align2::LEFT_CENTER,
                "LCARS",
                egui::FontId::proportional(25.0_f32),
                Color32::BLACK,
            );

            // Sidebar buttons

            let buttons = [
                ("SYSTEM", purple),
                ("NETWORK", blue),
                ("FILES", mauve),
                ("STATUS", orange),
            ];

            let mut button_y = 90.0_f32;

            for (name, color) in buttons {
                let rect = Rect::from_min_size(
                    Pos2::new(15.0_f32, button_y),
                    Vec2::new(140.0_f32, 48.0_f32),
                );

                let response = ui.allocate_rect(rect, egui::Sense::click());

                let button_color = if response.hovered() {
                    color.gamma_multiply(1.25_f32)
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
                    button_color,
                );

                ui.painter().text(
                    rect.center(),
                    egui::Align2::CENTER_CENTER,
                    name,
                    egui::FontId::proportional(15.0_f32),
                    Color32::BLACK,
                );

                button_y += 62.0_f32;
            }

            // -------------------------------------------------
            // MAIN HEADER
            // -------------------------------------------------

            let main_left = sidebar_width + 25.0_f32;

            let header_rect = Rect::from_min_size(
                Pos2::new(main_left, 15.0_f32),
                Vec2::new(
                    available.width() - main_left - 20.0_f32,
                    55.0_f32,
                ),
            );

            ui.painter().rect_filled(
                header_rect,
                CornerRadius {
                    nw: 25,
                    ne: 0,
                    sw: 0,
                    se: 25,
                },
                mauve,
            );

            ui.painter().text(
                Pos2::new(main_left + 25.0_f32, 42.0_f32),
                egui::Align2::LEFT_CENTER,
                "SYSTEM ACCESS",
                egui::FontId::proportional(28.0_f32),
                Color32::BLACK,
            );

            // -------------------------------------------------
            // LOGIN PANEL
            // -------------------------------------------------

            let login_rect = Rect::from_min_size(
                Pos2::new(main_left + 30.0_f32, 100.0_f32),
                Vec2::new(390.0_f32, 350.0_f32),
            );

            ui.painter().rect_filled(
                login_rect,
                CornerRadius {
                    nw: 25,
                    ne: 0,
                    sw: 25,
                    se: 0,
                },
                dark_panel,
            );

            ui.painter().text(
                Pos2::new(
                    login_rect.left() + 25.0_f32,
                    login_rect.top() + 35.0_f32,
                ),
                egui::Align2::LEFT_CENTER,
                "CREW IDENTIFICATION",
                egui::FontId::proportional(20.0_f32),
                orange,
            );

            // Username label

            ui.painter().text(
                Pos2::new(
                    login_rect.left() + 25.0_f32,
                    login_rect.top() + 85.0_f32,
                ),
                egui::Align2::LEFT_CENTER,
                "USER ID",
                egui::FontId::proportional(14.0_f32),
                white,
            );

            // Username field

            let username_rect = Rect::from_min_size(
                Pos2::new(
                    login_rect.left() + 25.0_f32,
                    login_rect.top() + 100.0_f32,
                ),
                Vec2::new(340.0_f32, 42.0_f32),
            );

            ui.allocate_new_ui(
                egui::UiBuilder::new().max_rect(username_rect),
                |ui| {
                    ui.add_sized(
                        [340.0_f32, 40.0_f32],
                        egui::TextEdit::singleline(&mut app.username)
                            .hint_text("Enter ID"),
                    );
                },
            );

            // Password label

            ui.painter().text(
                Pos2::new(
                    login_rect.left() + 25.0_f32,
                    login_rect.top() + 170.0_f32,
                ),
                egui::Align2::LEFT_CENTER,
                "PASSWORD",
                egui::FontId::proportional(14.0_f32),
                white,
            );

            // Password field

            let password_rect = Rect::from_min_size(
                Pos2::new(
                    login_rect.left() + 25.0_f32,
                    login_rect.top() + 185.0_f32,
                ),
                Vec2::new(340.0_f32, 42.0_f32),
            );

            ui.allocate_new_ui(
                egui::UiBuilder::new().max_rect(password_rect),
                |ui| {
                    ui.add_sized(
                        [340.0_f32, 40.0_f32],
                        egui::TextEdit::singleline(&mut app.password)
                            .password(true)
                            .hint_text("Enter password"),
                    );
                },
            );

            // -------------------------------------------------
            // ACCESS BUTTON
            // -------------------------------------------------

            let login_button_rect = Rect::from_min_size(
                Pos2::new(
                    login_rect.left() + 25.0_f32,
                    login_rect.top() + 250.0_f32,
                ),
                Vec2::new(340.0_f32, 48.0_f32),
            );

            let login_response =
                ui.allocate_rect(login_button_rect, egui::Sense::click());

            let login_color = if login_response.hovered() {
                orange.gamma_multiply(1.2_f32)
            } else {
                orange
            };

            ui.painter().rect_filled(
                login_button_rect,
                CornerRadius {
                    nw: 20,
                    ne: 0,
                    sw: 20,
                    se: 0,
                },
                login_color,
            );

            ui.painter().text(
                login_button_rect.center(),
                egui::Align2::CENTER_CENTER,
                "ACCESS SYSTEM",
                egui::FontId::proportional(17.0_f32),
                Color32::BLACK,
            );

            // -------------------------------------------------
            // LOGIN CHECK
            // -------------------------------------------------

            if login_response.clicked() {
                if app.username == "seaf" && app.password == "seaf123" {
                    app.login_message =
                        "ACCESS GRANTED — WELCOME".to_string();

                    app.screen = Screen::Home;
                } else {
                    app.login_message =
                        "ACCESS DENIED — INVALID CREDENTIALS".to_string();

                    app.password.clear();
                }
            }

            // Status message

            ui.painter().text(
                Pos2::new(
                    login_rect.left() + 25.0_f32,
                    login_rect.bottom() - 22.0_f32,
                ),
                egui::Align2::LEFT_CENTER,
                &app.login_message,
                egui::FontId::proportional(11.0_f32),
                blue,
            );

            // -------------------------------------------------
            // DECORATIVE RIGHT PANEL
            // -------------------------------------------------

            let right_panel = Rect::from_min_size(
                Pos2::new(main_left + 450.0_f32, 100.0_f32),
                Vec2::new(
                    available.width() - main_left - 490.0_f32,
                    350.0_f32,
                ),
            );

            ui.painter().rect_filled(
                right_panel,
                CornerRadius {
                    nw: 0,
                    ne: 30,
                    sw: 0,
                    se: 30,
                },
                Color32::from_rgb(30, 25, 45),
            );

            ui.painter().text(
                Pos2::new(
                    right_panel.center().x,
                    right_panel.top() + 35.0_f32,
                ),
                egui::Align2::CENTER_CENTER,
                "VESSEL STATUS",
                egui::FontId::proportional(20.0_f32),
                purple,
            );

            // Decorative circle

            let center = Pos2::new(
                right_panel.center().x,
                right_panel.center().y + 20.0_f32,
            );

            let radius = 80.0_f32;
            let segments = 64;

            let mut points = Vec::with_capacity(segments + 1);

            for i in 0..=segments {
                let angle =
                    (i as f32 / segments as f32) * std::f32::consts::TAU;

                points.push(Pos2::new(
                    center.x + radius * angle.cos(),
                    center.y + radius * angle.sin(),
                ));
            }

            ui.painter()
                .line(points, Stroke::new(3.0_f32, orange));

            ui.painter()
                .circle_filled(center, 12.0_f32, blue);

            ui.painter().text(
                center,
                egui::Align2::CENTER_CENTER,
                "01",
                egui::FontId::proportional(12.0_f32),
                Color32::BLACK,
            );

            // -------------------------------------------------
            // BOTTOM BAR
            // -------------------------------------------------

            let bottom_rect = Rect::from_min_size(
                Pos2::new(main_left, available.bottom() - 55.0_f32),
                Vec2::new(
                    available.width() - main_left - 20.0_f32,
                    40.0_f32,
                ),
            );

            ui.painter().rect_filled(
                bottom_rect,
                CornerRadius {
                    nw: 0,
                    ne: 20,
                    sw: 20,
                    se: 0,
                },
                purple,
            );

            ui.painter().text(
                bottom_rect.center(),
                egui::Align2::CENTER_CENTER,
                "LIVE LONG AND PROSPER",
                egui::FontId::proportional(14.0_f32),
                Color32::BLACK,
            );
        });
}
