
use eframe::egui;

use crate::main_window::Status::*;

use crate::main_window::MyApp;


pub fn initial_window(app: &mut MyApp, ctx: &egui::Context, frame: &mut eframe::Frame){
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.style_mut().visuals.override_text_color = Some(egui::Color32::WHITE);
            if ui.button("📷 Take").on_hover_text("Take a new capture").clicked(){
                frame.set_visible(false);
                app.disabled_time = ui.input(|i| i.time);
                app.prev = app.status;
                app.instant_flag = true;
                app.status = Hidden;
            }

            if app.screens.len()>1{
                if ui.button("📷 Take All").on_hover_text("Take a new capture of all the screens together").clicked(){
                    frame.set_visible(false);
                    app.disabled_time = ui.input(|i| i.time);
                    app.prev = app.status;
                    app.instant_flag = true;
                    app.status = Hidden;
                    app.all_screens = true;
                }
            }

            if ui.button("⏰ Delay").on_hover_text("Delay a new capture").clicked(){
                frame.set_visible(false);
                app.disabled_time = ui.input(|i| i.time);
                app.prev = app.status;
                app.instant_flag = false;
                app.status = Hidden;
            }

            if app.screens.len()>1{
                egui::ComboBox::from_label("").width(10.0)
                    .selected_text(format!("Screen: {}", app.sel_screen+1))
                    .show_ui(ui, |ui| {
                        for i in 0..app.screens.len(){
                            ui.selectable_value(&mut app.sel_screen, i, format!("{}", i+1));
                        }
                    });
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Max), |ui|{
                if ui.button("⚙").on_hover_text("General settings").clicked(){
                    app.prev = app.status;
                    app.status = Settings;
                }
            });
        });

        ui.vertical_centered(|ui|{
            ui.add(egui::TextEdit::singleline(&mut "Press Take to capture a new screenshot"));
        });
        if app.screens.len()>1{
            ui.vertical_centered(|ui|{
                ui.add(egui::TextEdit::multiline(&mut "Press Take All to capture a new screenshot from all the connected screens"));
            });
        }
    });
}
