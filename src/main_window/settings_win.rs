use std::path::Path;
use eframe::egui;
use eframe::egui::scroll_area::ScrollBarVisibility;
use keyboard_types::Code;
use crate::hotkey_popup::*;
use crate::image_proc::extensions::Extensions;
use crate::main_window::MyApp;

pub fn settings_window(app: &mut MyApp, ctx: &egui::Context, _frame: &mut eframe::Frame){
    egui::CentralPanel::default().show(ctx, |ui| {
        egui::ScrollArea::vertical()
            .scroll_bar_visibility(ScrollBarVisibility::VisibleWhenNeeded)
            .drag_to_scroll(false)
            .auto_shrink([false; 2])
            .show(ui, |ui|{
                ui.style_mut().visuals.override_text_color = Some(egui::Color32::WHITE);
                ui.heading("Settings Editor");
                let labels = ["New capture", "Delay capture", "Crop capture", "Draw capture", "Text capture", "Copy to clipboard", "Save capture"];
                for (i, l) in labels.iter().enumerate(){
                    ui.label(l.to_string());
                    ui.horizontal(|ui| {

                        ui.add_space(15.0);

                        let alt_label = ui.label("ALT: ");
                        ui.checkbox(app.hk_copy.get_shortcuts(i).get_mut_alt(), "")
                            .labelled_by(alt_label.id);
                        let shift_label = ui.label("SHIFT: ");
                        ui.checkbox(app.hk_copy.get_shortcuts(i).get_mut_shift(), "")
                            .labelled_by(shift_label.id);
                        let control_label = ui.label("CTRL: ");
                        ui.checkbox(app.hk_copy.get_shortcuts(i).get_mut_ctrl(), "")
                            .labelled_by(control_label.id);

                        egui::ComboBox::from_label(format!("KEY-CODE {}:", i)).width(5.0)
                            .selected_text(format!("{:?}", app.hk_copy.get_shortcuts(i).get_immut_selkey()))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(app.hk_copy.get_shortcuts(i).get_mut_selkey(), Code::KeyQ, "q");
                                ui.selectable_value(app.hk_copy.get_shortcuts(i).get_mut_selkey(), Code::KeyW, "w");
                                ui.selectable_value(app.hk_copy.get_shortcuts(i).get_mut_selkey(), Code::KeyE, "e");
                                ui.selectable_value(app.hk_copy.get_shortcuts(i).get_mut_selkey(), Code::KeyR, "r");
                                ui.selectable_value(app.hk_copy.get_shortcuts(i).get_mut_selkey(), Code::KeyT, "t");
                                ui.selectable_value(app.hk_copy.get_shortcuts(i).get_mut_selkey(), Code::KeyY, "y");
                                ui.selectable_value(app.hk_copy.get_shortcuts(i).get_mut_selkey(), Code::KeyU, "u");
                                ui.selectable_value(app.hk_copy.get_shortcuts(i).get_mut_selkey(), Code::KeyI, "i");
                                ui.selectable_value(app.hk_copy.get_shortcuts(i).get_mut_selkey(), Code::KeyO, "o");
                                ui.selectable_value(app.hk_copy.get_shortcuts(i).get_mut_selkey(), Code::KeyP, "p");
                                ui.selectable_value(app.hk_copy.get_shortcuts(i).get_mut_selkey(), Code::KeyA, "a");
                                ui.selectable_value(app.hk_copy.get_shortcuts(i).get_mut_selkey(), Code::KeyS, "s");
                                ui.selectable_value(app.hk_copy.get_shortcuts(i).get_mut_selkey(), Code::KeyD, "d");
                                ui.selectable_value(app.hk_copy.get_shortcuts(i).get_mut_selkey(), Code::KeyF, "f");
                                ui.selectable_value(app.hk_copy.get_shortcuts(i).get_mut_selkey(), Code::KeyG, "g");
                                ui.selectable_value(app.hk_copy.get_shortcuts(i).get_mut_selkey(), Code::KeyH, "h");
                                ui.selectable_value(app.hk_copy.get_shortcuts(i).get_mut_selkey(), Code::KeyJ, "j");
                                ui.selectable_value(app.hk_copy.get_shortcuts(i).get_mut_selkey(), Code::KeyK, "k");
                                ui.selectable_value(app.hk_copy.get_shortcuts(i).get_mut_selkey(), Code::KeyL, "l");
                                ui.selectable_value(app.hk_copy.get_shortcuts(i).get_mut_selkey(), Code::KeyZ, "z");
                                ui.selectable_value(app.hk_copy.get_shortcuts(i).get_mut_selkey(), Code::KeyX, "x");
                                ui.selectable_value(app.hk_copy.get_shortcuts(i).get_mut_selkey(), Code::KeyC, "c");
                                ui.selectable_value(app.hk_copy.get_shortcuts(i).get_mut_selkey(), Code::KeyV, "v");
                                ui.selectable_value(app.hk_copy.get_shortcuts(i).get_mut_selkey(), Code::KeyB, "b");
                                ui.selectable_value(app.hk_copy.get_shortcuts(i).get_mut_selkey(), Code::KeyN, "n");
                                ui.selectable_value(app.hk_copy.get_shortcuts(i).get_mut_selkey(), Code::KeyM, "m");
                            });

                        let (_id, _str, hotk) = app.hk_copy.get_shortcuts(i).id_gen();

                        /*if app.forbidden_hk[i]{
                            ui.scope(|ui|{
                                ui.style_mut().visuals.override_text_color = Some(egui::Color32::LIGHT_RED);
                                ui.label("Combination already in use; please select another one");
                            });
                        }*/

                        if i==1{
                            egui::ComboBox::from_label(format!("Capture Delay:",)).width(5.0)
                                .selected_text(format!("{}", app.delay_secs_cp))
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut app.delay_secs_cp, 3u32, "3");
                                    ui.selectable_value(&mut app.delay_secs_cp, 5u32, "5");
                                    ui.selectable_value(&mut app.delay_secs_cp, 10u32, "10");
                                });
                        }


                        if ui.add(egui::Button::new("OK")).clicked() {
                            let alt = app.hk_copy.get_shortcuts(i).get_immut_alt().clone();
                            let ctrl = app.hk_copy.get_shortcuts(i).get_immut_ctrl().clone();
                            let shift = app.hk_copy.get_shortcuts(i).get_immut_shift().clone();
                            let sel_key = app.hk_copy.get_shortcuts(i).get_immut_selkey().clone();

                            if i==1{
                                app.delay_secs = app.delay_secs_cp;
                            }

                            let hotkey_old = app.hk.get_shortcuts(i).get_immut_hotkey();

                            let op = Operation::new(hotk, l.to_string(), alt, shift, ctrl, sel_key);

                            match app.hk.shortcuts_replace(i, op) {
                                Ok(_) => {
                                    app.manager_hk.unregister(hotkey_old).unwrap();
                                    app.manager_hk.register(hotk).unwrap();
                                    if app.forbidden_hk[i]{
                                        app.forbidden_hk[i] = false;
                                    }
                                }
                                Err(_) => {
                                    app.forbidden_hk[i] = true;
                                }
                            }

                        }
                    });

                    if app.forbidden_hk[i]{
                        ui.scope(|ui|{
                            ui.style_mut().visuals.override_text_color = Some(egui::Color32::LIGHT_RED);
                            ui.label("Combination already in use; please select another one");
                        });
                    }

                }

                ui.add_space(10.0);
                ui.heading("Save Extension");
                ui.horizontal(|ui|{
                    ui.add_space(15.0);
                    egui::ComboBox::from_label(format!("Save Extension: ")).width(15.0)
                        .selected_text(format!("{:?}", app.extension_copy))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut app.extension_copy, Extensions::PNG, "PNG");
                            ui.selectable_value(&mut app.extension_copy, Extensions::JPG, "JPG");
                            ui.selectable_value(&mut app.extension_copy, Extensions::GIF, "GIF");
                        });

                    if ui.add(egui::Button::new("OK")).clicked() {
                        app.extension = app.extension_copy;
                    }
                });

                ui.add_space(10.0);
                ui.heading("Save Directory");
                ui.horizontal(|ui|{
                    ui.add_space(15.0);
                    ui.scope(|ui|{
                        ui.add(egui::TextEdit::singleline(&mut app.save_path_copy)).highlight();
                    });
                    let path = Path::new(&app.save_path_copy);
                    if !path.exists() || !path.is_dir(){
                        ui.scope(|ui|{
                            ui.style_mut().visuals.override_text_color = Some(egui::Color32::LIGHT_RED);
                            ui.label("INVALID PATH");
                        });
                    }
                    else{
                        if ui.add(egui::Button::new("OK")).clicked() {
                            app.save_path = app.save_path_copy.clone();
                        }
                    }
                });

                ui.add_space(15.0);


                if ui.add(egui::Button::new("Back")).clicked() {
                    app.hk_copy = HotKeyPopUp::initialize(app.hk.get_all_shortcuts());
                    app.forbidden_hk = vec![false; app.hk_num];
                    app.status = app.prev;
                    app.extension_copy = app.extension;
                    app.save_path_copy = app.save_path.clone();
                    app.delay_secs_cp = app.delay_secs;
                }
            });

    });
}