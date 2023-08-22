
use eframe::egui;
use eframe::egui::scroll_area::ScrollBarVisibility;
use eframe::egui::{UserAttentionType, Vec2};
use crate::image_proc::get_image_from_memory;

use crate::main_window::Status::*;

use crate::main_window::{DrawStatus, min_my, MyApp};

pub fn image_window(app: &mut MyApp, ctx: &egui::Context, frame: &mut eframe::Frame){
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

            if ui.button("⏰ Delay").on_hover_text("Delay a new capture").clicked(){
                frame.set_visible(false);
                app.disabled_time = ui.input(|i| i.time);
                app.prev = app.status;
                app.instant_flag = false;
                app.status = Hidden;
            }

            if ui.button("✂ Crop").on_hover_text("Crop the taken capture").clicked(){
                let w = app.image_to_save.as_ref().unwrap().width();
                let h = app.image_to_save.as_ref().unwrap().height();
                let blur = app.image_to_save.as_ref().unwrap().blur_area(0, 0, w, h);
                app.anchor_corner = Some(((0.0, 0.0), app.window_image_ratio));
                app.prev_mouse_pos = None;
                app.cur_mouse_pos = None;
                app.bl_ar = Some(blur);
                app.prev = app.status;
                app.backup_image = app.image.clone();
                app.backup_image_to_save = app.image_to_save.clone();
                app.status = Crop;
            }

            if ui.button("✏ Draw").on_hover_text("Draw over the capture").clicked(){
                app.draw_layer = Some(app.image_to_save.as_ref().unwrap().free_hand_draw_init());
                app.backup_image = app.image.clone();
                app.backup_image_to_save = app.image_to_save.clone();
                app.highlight = false;
                app.rubber = false;
                app.status = Draw;
                app.draw_status = DrawStatus::Draw;
            }

            if ui.button("🇹 Text").on_hover_text("Write some text over the capture").clicked(){
                app.backup_image = app.image.clone();
                app.backup_image_to_save = app.image_to_save.clone();
                app.prev = app.status;
                app.status = Text;
            }

            if ui.button("📋 Copy").on_hover_text("Copy the capture on clipboard").clicked(){
                app.image_to_save.as_ref().unwrap().copy_to_clipboard(&mut app.clipboard).unwrap();
            }

            if ui.button("💾 Save").on_hover_text("Save the capture").clicked(){
                let mut location = String::from(app.save_path.as_str());
                if cfg!(target_os = "windows"){
                    if !app.save_path.ends_with("\\"){
                        location.push_str("\\");
                    }
                    if app.save_name.len() == 0{
                        app.image_to_save.as_ref().unwrap().save_as(location.as_str(), "", app.extension).unwrap();
                    }else {
                        app.image_to_save.as_ref().unwrap().save_as(location.as_str(), app.save_name.as_str(), app.extension).unwrap();
                    }
                    app.save_name = String::new();
                }else if cfg!(target_os = "macos") || cfg!(target_os = "linux"){
                    if !app.save_path.ends_with("/"){
                        location.push_str("/");
                    }
                    if app.save_name.len() == 0{
                        app.image_to_save.as_ref().unwrap().save_as(location.as_str(), "", app.extension).unwrap();
                    }else {
                        app.image_to_save.as_ref().unwrap().save_as(location.as_str(), app.save_name.as_str(), app.extension).unwrap();
                    }
                    app.save_name = String::new();
                }else{
                    panic!(); // da gestire
                }
            }

            if ui.button("↺").on_hover_text("Rotate Counter-Clockwise").clicked(){
                app.image_to_save.as_mut().unwrap().rotate270cv();
                let di = app.image_to_save.as_ref().unwrap().show();
                app.image = Some(ctx.load_texture(
                    "my-image",
                    get_image_from_memory(di, 0, 0, 1, 1),
                    Default::default()
                ));
            }

            if ui.button("↻").on_hover_text("Rotate Clockwise").clicked(){
                app.image_to_save.as_mut().unwrap().rotate90cv();
                let di = app.image_to_save.as_ref().unwrap().show();
                app.image = Some(ctx.load_texture(
                    "my-image",
                    get_image_from_memory(di, 0, 0, 1, 1),
                    Default::default()
                ));
            }

            if ui.button("⬌").on_hover_text("Flip Horizontally").clicked(){
                app.image_to_save.as_mut().unwrap().flip_horizontally();
                let di = app.image_to_save.as_ref().unwrap().show();
                app.image = Some(ctx.load_texture(
                    "my-image",
                    get_image_from_memory(di, 0, 0, 1, 1),
                    Default::default()
                ));
            }

            if ui.button("⬍").on_hover_text("Flip Vertically").clicked(){
                app.image_to_save.as_mut().unwrap().flip_vertically();
                let di = app.image_to_save.as_ref().unwrap().show();
                app.image = Some(ctx.load_texture(
                    "my-image",
                    get_image_from_memory(di, 0, 0, 1, 1),
                    Default::default()
                ));
            }


            if app.screens.len()>1{
                egui::ComboBox::from_label("")
                    .selected_text(format!("Screen: {}", app.sel_screen+1))
                    .show_ui(ui, |ui| {
                        for i in 0..app.screens.len(){
                            ui.selectable_value(&mut app.sel_screen, i, format!("{}", i+1));
                        }
                    });
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Max), |ui|{
                if ui.button("⚙ Settings").on_hover_text("General settings").clicked(){
                    app.prev = app.status;
                    app.status = Settings;
                }
            });

        });

        // image logic (https://stackoverflow.com/questions/75728074/simplest-way-to-display-an-image-from-a-filepath)
        //let screen_size = app.screens[0].get_size();
        let window_size = Vec2::new(ctx.screen_rect().width()-5.0, ctx.screen_rect().height()-60.0);
        let image_size =  app.image.as_ref().unwrap().size_vec2();
        //println!("{:?}  {:?}", (image_size.x, image_size.y), (window_size.x, window_size.y));
        app.window_image_ratio = min_my(window_size.y/image_size.y, window_size.x/image_size.x);

        ui.vertical_centered(|ui|{
            ui.add(egui::Image::new(app.image.as_ref().unwrap(),
                                    app.image.as_ref().unwrap().size_vec2()*app.window_image_ratio));
        });

        ui.horizontal(|ui| {
            ui.style_mut().visuals.override_text_color = Some(egui::Color32::WHITE);
            ui.label("File Name: ");

            ui.style_mut().visuals.widgets.hovered.bg_stroke.color = egui::Color32::WHITE;
            ui.add(egui::TextEdit::singleline(&mut app.save_name)).highlight();

        });

    });
}