use eframe::egui;
use eframe::egui::{Vec2};
use crate::main_window::Status::*;
use crate::cursor_scaling::*;
use crate::image_proc::{get_image_from_memory};
use crate::main_window::{min_my, MyApp};


pub fn crop_window(app: &mut MyApp, ctx: &egui::Context, _frame: &mut eframe::Frame){
    egui::CentralPanel::default().show(ctx, |ui| {
        let window_size = Vec2::new(ctx.screen_rect().width()-5.0, ctx.screen_rect().height()-60.0);
        let image_size =  app.image.as_ref().unwrap().size_vec2();
        app.window_image_ratio = min_my(window_size.y/image_size.y, window_size.x/image_size.x);

        match ctx.input(|i| i.pointer.hover_pos()) {
            Some(pos) => {
                let offset = (ctx.screen_rect().width() - app.image.as_ref().unwrap().size_vec2().x * app.window_image_ratio) / 2.0;

                let ((x,y),(w,h)) = app.bl_ar.as_ref().unwrap().get_crop_data();
                let upleft = (x,y);
                let upright = (x+w,y);
                let downleft = (x,y+h);
                let downright = (x+w,y+h);

                let c1 = cursor_position(upleft, 1.0/app.window_image_ratio);
                let c1 = (c1.0 as f32, c1.1 as f32);
                let c2 = cursor_position(upright, 1.0/app.window_image_ratio);
                let c2 = (c2.0 as f32, c2.1 as f32);
                let c3 = cursor_position(downleft, 1.0/app.window_image_ratio);
                let c3 = (c3.0 as f32, c3.1 as f32);
                let c4 = cursor_position(downright, 1.0/app.window_image_ratio);
                let c4 = (c4.0 as f32, c4.1 as f32);

                // alto a sx
                if (pos.x - offset > c1.0 && pos.x - offset < c1.0+10.0) && (pos.y > c1.1 && pos.y < c1.1+20.0) {
                    //println!("Angolo!!");
                    if ctx.input(|i| i.pointer.any_pressed()) {
                        app.any_pressed = true;
                        app.corner = Some(Corner::UpLeft);
                        //println!("pressed");
                    }
                }
                //basso a sx
                else if (pos.x - offset > c3.0 && pos.x - offset < c3.0+10.0) && ((pos.y > c3.1 -10.0) && (pos.y < c3.1 + 10.0)) {
                    //println!("Angolo!!");
                    if ctx.input(|i| i.pointer.any_pressed()) {
                        app.any_pressed = true;
                        app.corner = Some(Corner::DownLeft);
                        //println!("pressed");
                    }
                }
                //alto a dx
                else if ((pos.x - offset > c2.0 - 10.0) && (pos.x - offset < c2.0 + 10.0)) && (pos.y > c2.1 && pos.y < c2.1+20.0) {
                    //println!("Angolo!!");
                    if ctx.input(|i| i.pointer.any_pressed()) {
                        app.corner = Some(Corner::UpRight);
                        app.any_pressed = true;
                        //println!("pressed");
                    }
                }
                //basso a dx
                else if ((pos.x - offset > c4.0 - 10.0) && (pos.x - offset < c4.0 + 10.0)) && ((pos.y > c4.1 - 10.0) && (pos.y < c4.1 + 10.0)) {
                    //println!("Angolo!!");
                    if ctx.input(|i| i.pointer.any_pressed()) {
                        app.corner = Some(Corner::DownRight);
                        app.any_pressed = true;
                        //println!("pressed");
                    }
                }

                if app.any_pressed {
                    match app.cur_mouse_pos {
                        None => {}
                        Some(p) => {
                            app.prev_mouse_pos = Some(p);
                        }
                    }

                    let (xr, yr) = cursor_position(((pos.x-offset) as u32, pos.y as u32), app.window_image_ratio);

                    app.cur_mouse_pos = Some((xr, yr));

                    match app.prev_mouse_pos {
                        None => {}
                        Some(_p) => {
                            let ((x, y), (w, h)) = app.bl_ar.as_ref().unwrap().get_crop_data();
                            //println!("{:?} {:?}", app.prev_mouse_pos.unwrap(), app.cur_mouse_pos.unwrap());

                            let ((xn, yn), (wn, hn)) = get_new_area(
                                app.prev_mouse_pos.unwrap(),
                                app.cur_mouse_pos.unwrap(),
                                (x, y),
                                (w, h),
                                (app.image_to_save.as_ref().unwrap().width(),app.image_to_save.as_ref().unwrap().height()),
                                app.corner.unwrap()
                            );

                            app.bl_ar.as_mut().unwrap().resize((xn, yn), (wn, hn));
                            let di = app.bl_ar.as_ref().unwrap().show(app.borders.as_ref().unwrap());

                            app.image = Some(ctx.load_texture(
                                "my-image",
                                get_image_from_memory(di, 0, 0, 1, 1),
                                Default::default()
                            ));
                        }
                    }

                    match app.corner.unwrap(){
                        Corner::UpLeft | Corner::UpRight | Corner::DownLeft => {
                            let (x,y)= cursor_position(((pos.x-offset) as u32, pos.y as u32), 1.0/app.window_image_ratio);
                            app.anchor_corner = Some(((x as f32, y as f32), app.window_image_ratio));
                        }
                        _ => {} //inutile
                    }
                }

                if ctx.input(|i| i.pointer.any_released()) && app.any_pressed {
                    //println!("released");
                    app.any_pressed = false;
                    app.corner = None;
                    app.prev_mouse_pos = None;
                    app.cur_mouse_pos = None;
                }
            }
            None => {}
        }

        /*ui.vertical_centered(|ui| {
            ui.add(egui::Image::new(app.image.as_ref().unwrap(),
                                    app.image.as_ref().unwrap().size_vec2() * app.window_image_ratio));
        });*/

        ui.vertical_centered(|ui| {

            let di = app.bl_ar.as_ref().unwrap().show(app.borders.as_ref().unwrap());

            app.image = Some(ctx.load_texture(
                "my-image",
                get_image_from_memory(di, 0, 0, 1, 1),
                Default::default()
            ));

            ui.add(egui::Image::new(app.image.as_ref().unwrap(),
                                    app.image.as_ref().unwrap().size_vec2() * app.window_image_ratio));
        });

        ui.horizontal(|ui|{
            ui.style_mut().visuals.override_text_color = Some(egui::Color32::WHITE);
            if ui.add(egui::Button::new("OK")).clicked() {
                app.prev = app.status;
                app.status = Image;
                app.last_crop_data = Some(app.bl_ar.as_ref().unwrap().get_crop_data());
                app.image_to_save.as_mut().unwrap().crop(app.bl_ar.take().unwrap());
                app.image = Some(ctx.load_texture(
                    "my-image",
                    get_image_from_memory(app.image_to_save.as_ref().unwrap().show(), 0, 0, 1, 1),
                    Default::default()
                ));

                if !app.all_images.is_empty(){
                    let _ = app.all_images.remove(app.sel_image);
                    let _ = app.all_images_to_save.remove(app.sel_image);
                    app.all_images_to_save.insert(app.sel_image, app.image_to_save.as_ref().unwrap().clone());
                    app.all_images.insert(app.sel_image, app.image.as_ref().unwrap().clone());
                }

            }

            if ui.add(egui::Button::new("Back")).clicked(){
                app.prev = app.status;
                app.status = Image;
                app.bl_ar = None;
                app.image_to_save = app.backup_image_to_save.clone();
                app.image = app.backup_image.clone();

                if !app.all_images.is_empty(){
                    let _ = app.all_images.remove(app.sel_image);
                    let _ = app.all_images_to_save.remove(app.sel_image);
                    app.all_images_to_save.insert(app.sel_image, app.image_to_save.as_ref().unwrap().clone());
                    app.all_images.insert(app.sel_image, app.image.as_ref().unwrap().clone());
                }
            }
        });


    });

}