
use eframe::egui;
use eframe::egui::scroll_area::ScrollBarVisibility;
use eframe::egui::{UserAttentionType, Vec2};

use crate::main_window::Status::*;
use crate::{image_proc, screensh};
use crate::cursor_scaling::*;

use crate::image_proc::{get_image, load_image_from_memory, get_image_from_memory};

use crate::main_window::{min_my, MyApp};

pub fn text_window(app: &mut MyApp, ctx: &egui::Context, frame: &mut eframe::Frame){
    egui::CentralPanel::default().show(ctx, |ui| {
        egui::ScrollArea::vertical()
            .scroll_bar_visibility(ScrollBarVisibility::AlwaysVisible)
            .drag_to_scroll(false)
            .auto_shrink([true; 2])
            .show(ui, |ui|{
                ui.style_mut().visuals.override_text_color = Some(egui::Color32::WHITE);
                app.dropdown_on = false;

                let window_size = Vec2::new(ctx.screen_rect().width() - 5.0, ctx.screen_rect().height() - 60.0);
                let image_size = app.backup_image.as_ref().unwrap().size_vec2();
                app.window_image_ratio = min_my(window_size.y / image_size.y, window_size.x / image_size.x);
                if window_size.y / image_size.y < window_size.x / image_size.x{
                    app.is_ratio_along_y = true;
                }
                else{
                    app.is_ratio_along_y = false;
                }

                let offset = (ctx.screen_rect().width() - app.backup_image.as_ref().unwrap().size_vec2().x * app.window_image_ratio) / 2.0 - 5.0;

                match app.sel_font.as_ref() {
                    Some(k) => app.sel_font = Some(k.clone()),
                    None => app.sel_font = Some(app.fonts.as_ref().unwrap().iter().nth(0).unwrap().0.to_string())
                }

                ui.horizontal(|ui| {
                    egui::ComboBox::from_label(format!("Font"))
                        .selected_text(format!("{}", app.sel_font.as_ref().unwrap()))
                        .show_ui(ui, |ui| {
                            app.dropdown_on = true;
                            for (s, f) in app.fonts.as_ref().unwrap() {
                                ui.selectable_value(&mut app.sel_font, Some(s.clone()), s);
                            }
                        });

                    egui::ComboBox::from_label(format!("Size"))
                        .selected_text(format!("{}", app.sel_font_size))
                        .show_ui(ui, |ui| {
                            app.dropdown_on = true;
                            for i in (10..30).step_by(2) {
                                ui.selectable_value(&mut app.sel_font_size, i, format!("{i}"));
                            }
                        });

                    ui.style_mut().visuals.widgets.hovered.bg_stroke.color = egui::Color32::WHITE;
                    ui.add(egui::TextEdit::singleline(&mut app.image_text)).highlight();

                });

                ui.horizontal(|ui|{
                    if !app.is_sel_color && ui.add(egui::Button::new("Edit Color")).clicked() {
                        app.is_sel_color = true;
                    }

                    if app.is_sel_color {
                        app.dropdown_on = true;
                        let mut color_vec = [app.sel_color.color.0[0], app.sel_color.color.0[1], app.sel_color.color.0[2]];
                        egui::widgets::color_picker::color_edit_button_srgb(ui, &mut color_vec);

                        app.sel_color.color.0[0] = color_vec[0];
                        app.sel_color.color.0[1] = color_vec[1];
                        app.sel_color.color.0[2] = color_vec[2];

                        if ui.add(egui::Button::new("OK")).clicked() {
                            app.is_sel_color = false;
                        }
                    }
                });

                match ctx.input(|i| i.pointer.hover_pos()) {
                    None => {}
                    Some(pos) => {
                        let scroll = ctx.input(|i| i.scroll_delta).y;
                        //println!("{:?}", scroll);

                        if app.is_ratio_along_y || (!app.is_ratio_along_y && (app.window_image_ratio>0.215 && app.window_image_ratio<0.23)){
                            app.scroll_qty = app.scroll_qty - scroll;
                            if app.scroll_qty < 0.0 {
                                app.scroll_qty = 0.0;
                            }
                            if app.scroll_qty > 20.0 {
                                app.scroll_qty = 20.0;
                            }
                        }
                        else{
                            app.scroll_qty = 0.0;
                        }
                        //println!("{}", app.window_image_ratio);
                        //println!("{:?}", app.is_ratio_along_y);

                        if pos.x - offset > 0.0 && pos.x - offset < image_size.x * app.window_image_ratio
                            && pos.y+app.scroll_qty > 51.0 && pos.y+app.scroll_qty < (image_size.y * app.window_image_ratio + 51.0) && !app.any_pressed && !app.dropdown_on {
                            //println!("Dentro");
                            match ctx.input(|i| i.pointer.any_pressed()) {
                                true => {
                                    app.any_pressed = true;
                                    //let start = (pos.x as i32, pos.y as i32);

                                    let start = cursor_position(((pos.x-offset) as u32, (pos.y-60.0+app.scroll_qty) as u32), app.window_image_ratio);
                                    let start = (start.0 as i32, start.1 as i32);
                                    app.backup_image_to_save.as_mut().unwrap().put_text(
                                        start,
                                        &app.sel_color,
                                        app.image_text.as_str(),
                                        (app.sel_font_size as f32)*5.0,/**app.window_image_ratio*20.0,*/
                                        app.fonts.as_ref().unwrap().get(app.sel_font.as_ref().unwrap().as_str()).unwrap()
                                    );
                                    app.any_pressed = false;
                                    app.backup_image = Some(ctx.load_texture(
                                        "my-image",
                                        get_image_from_memory(app.backup_image_to_save.as_ref().unwrap().show(), 0, 0, 1, 1),
                                        Default::default()
                                    ));
                                }
                                false => {}
                            }
                        }
                    }
                }

                ui.vertical_centered(|ui| {
                    ui.add(egui::Image::new(app.backup_image.as_ref().unwrap(),
                                            app.backup_image.as_ref().unwrap().size_vec2() * app.window_image_ratio));
                });


                ui.horizontal(|ui| {
                    if ui.add(egui::Button::new("OK")).clicked() {
                        app.status = app.prev;
                        app.prev = Text;
                        app.any_pressed = false;
                        app.image_text = String::from("Insert text here");
                        //app.image_to_save.as_mut().unwrap().crop(app.bl_ar.take().unwrap());
                        app.backup_image = Some(ctx.load_texture(
                            "my-image",
                            get_image_from_memory(app.backup_image_to_save.as_ref().unwrap().show(), 0, 0, 1, 1),
                            Default::default()
                        ));

                        app.image = app.backup_image.clone();
                        app.image_to_save = app.backup_image_to_save.clone();

                    }

                    if ui.add(egui::Button::new("Undo")).clicked(){
                        app.backup_image_to_save.as_mut().unwrap().undo();
                        let di = app.backup_image_to_save.as_ref().unwrap().show();
                        app.backup_image = Some(ctx.load_texture(
                            "my-image",
                            get_image_from_memory(di, 0, 0, 1, 1),
                            Default::default()
                        ));
                        app.image = app.backup_image.clone();
                        app.image_to_save = app.backup_image_to_save.clone();
                    }

                    if ui.add(egui::Button::new("Back")).clicked() {
                        app.any_pressed = false;
                        app.backup_image = app.image.clone();
                        app.backup_image_to_save = app.image_to_save.clone();
                        app.status = Image;
                    }
                });
            });
    });
}