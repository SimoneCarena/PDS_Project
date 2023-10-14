use eframe::egui;
use eframe::egui::scroll_area::ScrollBarVisibility;
use eframe::egui::{Vec2};
use crate::main_window::Status::*;
use crate::cursor_scaling::*;
use crate::image_proc::{get_image_from_memory};
use crate::image_proc::Image;
use crate::main_window::{DrawStatus, min_my, MyApp, Pointing, Shape};

// si usa backup

pub fn draw_window(app: &mut MyApp, ctx: &egui::Context, _frame: &mut eframe::Frame){

    egui::CentralPanel::default().show(ctx, |ui| {
        egui::ScrollArea::vertical()
            .scroll_bar_visibility(ScrollBarVisibility::AlwaysVisible)
            .drag_to_scroll(false)
            .auto_shrink([true; 2])
            .show(ui, |ui| {
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


                ui.horizontal(|ui| {
                    ui.style_mut().visuals.override_text_color = Some(egui::Color32::WHITE);
                    if ui.button("✏ Draw").on_hover_text("Free-hand drawing").clicked() {
                        app.rubber = false;
                        if app.draw_layer.is_some() && app.rubber_layer.is_some() {
                            match app.draw_status {
                                DrawStatus::Shape(1) => {
                                    app.backup_image_to_save.as_mut().unwrap().shape_set(app.rubber_layer.take().unwrap(), app.draw_layer.take().unwrap());
                                    let di = app.image_to_save.as_ref().unwrap().show();
                                    app.backup_image = Some(ctx.load_texture(
                                        "my-image",
                                        get_image_from_memory(di, 0, 0, 1, 1),
                                        Default::default()
                                    ));
                                }
                                _ => {}
                            }
                        }

                        let dl = app.backup_image_to_save.as_ref().unwrap().free_hand_draw_init();
                        app.draw_layer = Some(dl);
                        app.draw_status = DrawStatus::Draw;
                    }
                    if ui.button("🗑 Erase").on_hover_text("Erase annotations").clicked() {
                        app.rubber = true;
                        if app.draw_layer.is_some() && app.rubber_layer.is_some() {
                            match app.draw_status {
                                DrawStatus::Shape(1) => {
                                    app.backup_image_to_save.as_mut().unwrap().shape_set(app.rubber_layer.take().unwrap(), app.draw_layer.take().unwrap());
                                    let di = app.image_to_save.as_ref().unwrap().show();
                                    app.backup_image = Some(ctx.load_texture(
                                        "my-image",
                                        get_image_from_memory(di, 0, 0, 1, 1),
                                        Default::default()
                                    ));
                                }
                                _ => {}
                            }
                        }

                        let (rl, dl) = app.backup_image_to_save.as_ref().unwrap().rubber_init(app.last_crop_data);
                        app.rubber_layer = Some(rl);
                        app.draw_layer = Some(dl);
                        app.draw_status = DrawStatus::Rubber;
                    }
                    if ui.button("📌 Highlight").on_hover_text("Activate highlighter").clicked() {
                        app.rubber = false;
                        if app.draw_layer.is_some() && app.rubber_layer.is_some() {
                            match app.draw_status {
                                DrawStatus::Shape(1) => {
                                    app.backup_image_to_save.as_mut().unwrap().shape_set(app.rubber_layer.take().unwrap(), app.draw_layer.take().unwrap());
                                    let di = app.backup_image_to_save.as_ref().unwrap().show();
                                    app.backup_image = Some(ctx.load_texture(
                                        "my-image",
                                        get_image_from_memory(di, 0, 0, 1, 1),
                                        Default::default()
                                    ));
                                }
                                _ => {}
                            }
                        }
                        let (rl, dl) = app.backup_image_to_save.as_ref().unwrap().highlight_init();
                        app.rubber_layer = Some(rl);
                        app.draw_layer = Some(dl);
                        app.draw_status = DrawStatus::Highlight;
                    }

                    match app.draw_status{
                        DrawStatus::Draw | DrawStatus::Rubber => {
                            ui.add(egui::Slider::new(&mut app.pencil_rubber_thickness, 1..=60).text("Trait size"));
                        }
                        DrawStatus::Highlight => {
                            ui.add(egui::Slider::new(&mut app.highlight_thickness, 20..=80).text("Trait size"));
                        }
                        DrawStatus::Shape(_) => {}
                    }

                });
                ui.horizontal(|ui| {
                    ui.label("SHAPES");
                    if ui.button("⬛").on_hover_text("Filled rectangle").clicked() {
                        if app.draw_layer.is_some() && app.rubber_layer.is_some() {
                            match app.draw_status {
                                DrawStatus::Shape(1) => {
                                    app.backup_image_to_save.as_mut().unwrap().shape_set(app.rubber_layer.take().unwrap(), app.draw_layer.take().unwrap());
                                    let di = app.backup_image_to_save.as_ref().unwrap().show();
                                    app.backup_image = Some(ctx.load_texture(
                                        "my-image",
                                        get_image_from_memory(di, 0, 0, 1, 1),
                                        Default::default()
                                    ));
                                }
                                _ => {}
                            }
                        }
                        app.which_shape = Some(Shape::FilledRectangle);
                        app.draw_status = DrawStatus::Shape(0);
                    }

                    if ui.button("⬜").on_hover_text("Empty rectangle").clicked() {
                        if app.draw_layer.is_some() && app.rubber_layer.is_some() {
                            match app.draw_status {
                                DrawStatus::Shape(1) => {
                                    app.backup_image_to_save.as_mut().unwrap().shape_set(app.rubber_layer.take().unwrap(), app.draw_layer.take().unwrap());
                                    let di = app.backup_image_to_save.as_ref().unwrap().show();
                                    app.backup_image = Some(ctx.load_texture(
                                        "my-image",
                                        get_image_from_memory(di, 0, 0, 1, 1),
                                        Default::default()
                                    ));
                                }
                                _ => {}
                            }
                        }
                        app.which_shape = Some(Shape::EmptyRectangle);
                        app.draw_status = DrawStatus::Shape(0);
                    }

                    if ui.button("⏺").on_hover_text("Filled circle").clicked() {
                        if app.draw_layer.is_some() && app.rubber_layer.is_some() {
                            match app.draw_status {
                                DrawStatus::Shape(1) => {
                                    app.backup_image_to_save.as_mut().unwrap().shape_set(app.rubber_layer.take().unwrap(), app.draw_layer.take().unwrap());
                                    let di = app.backup_image_to_save.as_ref().unwrap().show();
                                    app.backup_image = Some(ctx.load_texture(
                                        "my-image",
                                        get_image_from_memory(di, 0, 0, 1, 1),
                                        Default::default()
                                    ));
                                }
                                _ => {}
                            }
                        }
                        app.which_shape = Some(Shape::FilledCircle);
                        app.draw_status = DrawStatus::Shape(0);
                    }

                    if ui.button("○").on_hover_text("Empty circle").clicked() {
                        if app.draw_layer.is_some() && app.rubber_layer.is_some() {
                            match app.draw_status {
                                DrawStatus::Shape(1) => {
                                    app.backup_image_to_save.as_mut().unwrap().shape_set(app.rubber_layer.take().unwrap(), app.draw_layer.take().unwrap());
                                    let di = app.backup_image_to_save.as_ref().unwrap().show();
                                    app.backup_image = Some(ctx.load_texture(
                                        "my-image",
                                        get_image_from_memory(di, 0, 0, 1, 1),
                                        Default::default()
                                    ));
                                }
                                _ => {}
                            }
                        }
                        app.which_shape = Some(Shape::EmptyCircle);
                        app.draw_status = DrawStatus::Shape(0);
                    }

                    if ui.button("⬅").on_hover_text("Left arrow").clicked() {
                        if app.draw_layer.is_some() && app.rubber_layer.is_some() {
                            match app.draw_status {
                                DrawStatus::Shape(1) => {
                                    app.backup_image_to_save.as_mut().unwrap().shape_set(app.rubber_layer.take().unwrap(), app.draw_layer.take().unwrap());
                                    let di = app.backup_image_to_save.as_ref().unwrap().show();
                                    app.backup_image = Some(ctx.load_texture(
                                        "my-image",
                                        get_image_from_memory(di, 0, 0, 1, 1),
                                        Default::default()
                                    ));
                                }
                                _ => {}
                            }
                        }
                        app.which_shape = Some(Shape::Arrow(Pointing::Left));
                        app.draw_status = DrawStatus::Shape(0);
                    }

                    if ui.button("➡").on_hover_text("Right arrow").clicked() {
                        if app.draw_layer.is_some() && app.rubber_layer.is_some() {
                            match app.draw_status {
                                DrawStatus::Shape(1) => {
                                    app.backup_image_to_save.as_mut().unwrap().shape_set(app.rubber_layer.take().unwrap(), app.draw_layer.take().unwrap());
                                    let di = app.backup_image_to_save.as_ref().unwrap().show();
                                    app.backup_image = Some(ctx.load_texture(
                                        "my-image",
                                        get_image_from_memory(di, 0, 0, 1, 1),
                                        Default::default()
                                    ));
                                }
                                _ => {}
                            }
                        }
                        app.which_shape = Some(Shape::Arrow(Pointing::Right));
                        app.draw_status = DrawStatus::Shape(0);
                    }

                    if ui.button("⬆").on_hover_text("Up arrow").clicked() {
                        if app.draw_layer.is_some() && app.rubber_layer.is_some() {
                            match app.draw_status {
                                DrawStatus::Shape(1) => {
                                    app.backup_image_to_save.as_mut().unwrap().shape_set(app.rubber_layer.take().unwrap(), app.draw_layer.take().unwrap());
                                    let di = app.backup_image_to_save.as_ref().unwrap().show();
                                    app.backup_image = Some(ctx.load_texture(
                                        "my-image",
                                        get_image_from_memory(di, 0, 0, 1, 1),
                                        Default::default()
                                    ));
                                }
                                _ => {}
                            }
                        }
                        app.which_shape = Some(Shape::Arrow(Pointing::Up));
                        app.draw_status = DrawStatus::Shape(0);
                    }

                    if ui.button("⬇").on_hover_text("Down arrow").clicked() {
                        if app.draw_layer.is_some() && app.rubber_layer.is_some() {
                            match app.draw_status {
                                DrawStatus::Shape(1) => {
                                    app.backup_image_to_save.as_mut().unwrap().shape_set(app.rubber_layer.take().unwrap(), app.draw_layer.take().unwrap());
                                    let di = app.backup_image_to_save.as_ref().unwrap().show();
                                    app.backup_image = Some(ctx.load_texture(
                                        "my-image",
                                        get_image_from_memory(di, 0, 0, 1, 1),
                                        Default::default()
                                    ));
                                }
                                _ => {}
                            }
                        }
                        app.which_shape = Some(Shape::Arrow(Pointing::Down));
                        app.draw_status = DrawStatus::Shape(0);
                    }


                    if !app.rubber && !app.is_sel_color && ui.add(egui::Button::new("Edit Color")).clicked() {
                        app.is_sel_color = true;
                    }

                    if app.is_sel_color {
                        app.dropdown_on = true;

                        match app.draw_status{
                            DrawStatus::Draw |  DrawStatus::Shape(_) | DrawStatus::Rubber=> {
                                let mut color_vec = [app.draw_color.color.0[0], app.draw_color.color.0[1], app.draw_color.color.0[2]];
                                egui::widgets::color_picker::color_edit_button_srgb(ui, &mut color_vec);

                                app.draw_color.color.0[0] = color_vec[0];
                                app.draw_color.color.0[1] = color_vec[1];
                                app.draw_color.color.0[2] = color_vec[2];
                            }
                            DrawStatus::Highlight => {
                                let mut color_vec = [app.highlight_color.color.0[0], app.highlight_color.color.0[1], app.highlight_color.color.0[2]];
                                egui::widgets::color_picker::color_edit_button_srgb(ui, &mut color_vec);

                                app.highlight_color.color.0[0] = color_vec[0];
                                app.highlight_color.color.0[1] = color_vec[1];
                                app.highlight_color.color.0[2] = color_vec[2];
                            }
                        }



                        if ui.add(egui::Button::new("OK")).clicked() {
                            app.is_sel_color = false;
                            app.dropdown_on = false;
                        }
                    }
                });

                let di;
                let offset = (ctx.screen_rect().width() - app.backup_image.as_ref().unwrap().size_vec2().x * app.window_image_ratio) / 2.0 -5.0;
                match ctx.input(|i| i.pointer.hover_pos()) {
                    None => {}
                    Some(pos) => {
                        let scroll = ctx.input(|i| i.scroll_delta).y;
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

                        match app.draw_status {
                            DrawStatus::Draw | DrawStatus::Highlight | DrawStatus::Rubber => {
                                if pos.x - offset > 0.0 && pos.x - offset < image_size.x * app.window_image_ratio
                                    && pos.y+app.scroll_qty > 50.0 && pos.y+app.scroll_qty < (image_size.y * app.window_image_ratio + 50.0) && !app.dropdown_on {
                                    let scaled_pos = cursor_position(((pos.x - offset) as u32, (pos.y+app.scroll_qty - 50.0) as u32), app.window_image_ratio);
                                    app.cur_mouse_pos = Some(scaled_pos);
                                    let cur = app.cur_mouse_pos.unwrap().clone();

                                    match ctx.input(|i| i.pointer.any_pressed()) && !app.dropdown_on {
                                        true => {
                                            app.any_pressed = true;
                                        },
                                        false => {}
                                    }

                                    let di;
                                    if app.any_pressed {
                                        match app.draw_status {
                                            DrawStatus::Draw => {
                                                app.prev_edge = Some(Image::draw_point(app.draw_layer.as_mut().unwrap(), app.prev_edge.clone(), (cur.0 as i32, cur.1 as i32), (app.pencil_rubber_thickness as f32) as i32, &app.draw_color));
                                                di = app.draw_layer.as_ref().unwrap().show();
                                                app.backup_image = Some(ctx.load_texture("my-image", get_image_from_memory(di, 0, 0, 1, 1), Default::default()));
                                            },
                                            DrawStatus::Rubber => {
                                                app.prev_edge = Some(Image::rubber(app.draw_layer.as_mut().unwrap(), app.prev_edge.clone(), (cur.0 as i32, cur.1 as i32), app.pencil_rubber_thickness));
                                                di = app.draw_layer.as_ref().unwrap().show_rubber(app.rubber_layer.as_ref().unwrap());
                                                app.backup_image = Some(ctx.load_texture("my-image", get_image_from_memory(di, 0, 0, 1, 1), Default::default()));
                                            },
                                            DrawStatus::Highlight => {
                                                app.prev_edge = Some(Image::highlight(app.draw_layer.as_mut().unwrap(), app.prev_edge.clone(), (cur.0 as i32, cur.1 as i32), (app.highlight_thickness as f32) as i32, &app.highlight_color));
                                                di = app.draw_layer.as_ref().unwrap().show_higlight(app.rubber_layer.as_ref().unwrap());
                                                app.backup_image = Some(ctx.load_texture("my-image", get_image_from_memory(di, 0, 0, 1, 1), Default::default()));
                                            },
                                            _ => {}
                                        }
                                    }

                                    if ctx.input(|i| i.pointer.any_released()) && app.any_pressed {
                                        app.any_pressed = false;
                                        match app.draw_status {
                                            DrawStatus::Draw => {
                                                app.backup_image_to_save.as_mut().unwrap().free_hand_draw_set(app.draw_layer.take().unwrap(), app.prev_edge.unwrap().clone().2, (app.pencil_rubber_thickness as f32 *1.5) as i32, &app.draw_color);
                                                app.draw_layer = Some(app.backup_image_to_save.as_ref().unwrap().free_hand_draw_init());
                                            },
                                            DrawStatus::Rubber => {
                                                app.backup_image_to_save.as_mut().unwrap().rubber_set(app.draw_layer.take().unwrap(), app.rubber_layer.as_ref().unwrap(), app.prev_edge.unwrap().clone().2, app.pencil_rubber_thickness*2);
                                                let (rl, dl) = app.backup_image_to_save.as_ref().unwrap().rubber_init(app.last_crop_data);
                                                app.rubber_layer = Some(rl);
                                                app.draw_layer = Some(dl);
                                            },
                                            DrawStatus::Highlight => {
                                                app.backup_image_to_save.as_mut().unwrap().highlight_set(app.draw_layer.take().unwrap(), app.rubber_layer.as_ref().unwrap(), app.prev_edge.unwrap().clone().2, (app.highlight_thickness as f32 * 1.5) as i32, &app.highlight_color);
                                                let (rl, dl) = app.backup_image_to_save.as_ref().unwrap().highlight_init();
                                                app.rubber_layer = Some(rl);
                                                app.draw_layer = Some(dl);
                                            }
                                            _ => {}
                                        }
                                        app.prev_edge = None;
                                    }
                                }
                            },
                            DrawStatus::Shape(click) => {
                                match click {
                                    0 => {
                                        if pos.x - offset > 0.0 && pos.x - offset < image_size.x * app.window_image_ratio
                                            && pos.y+app.scroll_qty > 50.0 && pos.y+app.scroll_qty < (image_size.y * app.window_image_ratio + 50.0) && !app.dropdown_on { //&& !app.any_pressed
                                            match ctx.input(|i| i.pointer.any_click()) {
                                                true => {
                                                    app.any_pressed = true;
                                                    let mut start = cursor_position(((pos.x - offset) as u32, (pos.y+app.scroll_qty - 50.0) as u32), app.window_image_ratio);

                                                    // controllo inizio rettangolo
                                                    if start.0 < 150 {
                                                        start.0 = 150;
                                                    } else if start.0 > (image_size.x - 150.0) as u32 {
                                                        start.0 = (image_size.x - 150.0) as u32;
                                                    }

                                                    if start.1 < 100 {
                                                        start.1 = 100;
                                                    } else if start.1 > (image_size.y - 100.0) as u32 {
                                                        start.1 = (image_size.y - 100.0) as u32;
                                                    }

                                                    let (rl, dl) = app.backup_image_to_save.as_ref().unwrap().shape_init(start, (300, 200));
                                                    app.rubber_layer = Some(rl);
                                                    app.draw_layer = Some(dl);

                                                    match app.which_shape.unwrap() {
                                                        Shape::FilledRectangle => Image::draw_filled_rectangle(app.draw_layer.as_mut().unwrap(), app.rubber_layer.as_mut().unwrap(), (start.0 as i32, start.1 as i32), (300, 200), &app.draw_color),
                                                        Shape::EmptyRectangle => Image::draw_empty_rectangle(app.draw_layer.as_mut().unwrap(), app.rubber_layer.as_mut().unwrap(), (start.0 as i32, start.1 as i32), (300, 200), &app.draw_color, app.pencil_rubber_thickness),
                                                        Shape::FilledCircle => Image::draw_filled_circle(app.draw_layer.as_mut().unwrap(), app.rubber_layer.as_mut().unwrap(), (start.0 as i32, start.1 as i32), 200, &app.draw_color),
                                                        Shape::EmptyCircle => Image::draw_empty_circle(app.draw_layer.as_mut().unwrap(), app.rubber_layer.as_mut().unwrap(), (start.0 as i32, start.1 as i32), 200, &app.draw_color, app.pencil_rubber_thickness),
                                                        Shape::Arrow(dir) => match dir {
                                                            Pointing::Left => Image::draw_filled_left_arrow(app.draw_layer.as_mut().unwrap(), app.rubber_layer.as_mut().unwrap(), (start.0 as i32, start.1 as i32), (300, 200), &app.draw_color),
                                                            Pointing::Right => Image::draw_filled_right_arrow(app.draw_layer.as_mut().unwrap(), app.rubber_layer.as_mut().unwrap(), (start.0 as i32, start.1 as i32), (300, 200), &app.draw_color),
                                                            Pointing::Up => Image::draw_filled_up_arrow(app.draw_layer.as_mut().unwrap(), app.rubber_layer.as_mut().unwrap(), (start.0 as i32, start.1 as i32), (300, 200), &app.draw_color),
                                                            Pointing::Down => Image::draw_filled_down_arrow(app.draw_layer.as_mut().unwrap(), app.rubber_layer.as_mut().unwrap(), (start.0 as i32, start.1 as i32), (300, 200), &app.draw_color),
                                                        }
                                                    }

                                                    di = app.draw_layer.as_ref().unwrap().show_shape(app.rubber_layer.as_ref().unwrap());
                                                    app.backup_image = Some(
                                                        ctx.load_texture("my-image",
                                                                         get_image_from_memory(di, 0, 0, 1, 1),
                                                                         Default::default()
                                                        ));
                                                }
                                                false => {}
                                            }

                                            match ctx.input(|i| i.pointer.any_released()) && app.any_pressed {
                                                true => {
                                                    app.draw_status = DrawStatus::Shape(1);
                                                    app.any_pressed = false;
                                                }
                                                _ => {}
                                            }
                                        }
                                    },
                                    1 => {
                                        let ((x, y), (w, h)) = app.draw_layer.as_ref().unwrap().get_pos_size().unwrap();
                                        let upleft = (x, y);
                                        let upright = (x + w, y);
                                        let downleft = (x, y + h);
                                        let downright = (x + w, y + h);

                                        let c1 = cursor_position(upleft, 1.0 / app.window_image_ratio);
                                        let c1 = (c1.0 as f32, c1.1 as f32);
                                        let c2 = cursor_position(upright, 1.0 / app.window_image_ratio);
                                        let c2 = (c2.0 as f32, c2.1 as f32);
                                        let c3 = cursor_position(downleft, 1.0 / app.window_image_ratio);
                                        let c3 = (c3.0 as f32, c3.1 as f32);
                                        let c4 = cursor_position(downright, 1.0 / app.window_image_ratio);
                                        let c4 = (c4.0 as f32, c4.1 as f32);

                                        //alto sx
                                        if (pos.x - offset > c1.0 -10.0 && pos.x - offset < c1.0 + 10.0) && (pos.y+app.scroll_qty - 50.0 > c1.1-10.0 && pos.y+app.scroll_qty - 50.0 < c1.1 + 10.0) {
                                            if ctx.input(|i| i.pointer.any_pressed()) {
                                                app.any_pressed = true;
                                                app.corner = Some(Corner::UpLeft);
                                            }
                                        }
                                        //basso a sx
                                        else if (pos.x - offset > c3.0 -10.0 && pos.x - offset < c3.0 + 10.0) && ((pos.y+app.scroll_qty - 50.0 > c3.1 - 10.0) && (pos.y+app.scroll_qty - 50.0 < c3.1 + 10.0)) {
                                            if ctx.input(|i| i.pointer.any_pressed()) {
                                                app.any_pressed = true;
                                                app.corner = Some(Corner::DownLeft);
                                            }
                                        }
                                        //alto a dx
                                        else if ((pos.x - offset > c2.0 - 10.0) && (pos.x - offset < c2.0 + 10.0)) && (pos.y+app.scroll_qty - 50.0 > c2.1 -10.0 && pos.y+app.scroll_qty - 50.0 < c2.1 + 10.0) {
                                            if ctx.input(|i| i.pointer.any_pressed()) {
                                                app.corner = Some(Corner::UpRight);
                                                app.any_pressed = true;
                                            }
                                        }
                                        //basso a dx
                                        else if ((pos.x - offset > c4.0 - 10.0) && (pos.x - offset < c4.0 + 10.0)) && ((pos.y+app.scroll_qty - 50.0 > c4.1 - 10.0) && (pos.y+app.scroll_qty - 50.0 < c4.1 + 10.0)) {
                                            if ctx.input(|i| i.pointer.any_pressed()) {
                                                app.corner = Some(Corner::DownRight);
                                                app.any_pressed = true;
                                            }
                                        }
                                        //centro
                                        else if ((pos.x - offset > c1.0 + 10.0) && (pos.x - offset < c4.0 - 10.0)) && ((pos.y+app.scroll_qty - 50.0 > c1.1 +10.0) && (pos.y+app.scroll_qty - 50.0 < c4.1 - 10.0)) {
                                            if ctx.input(|i| i.pointer.any_pressed()) {
                                                app.corner = Some(Corner::Centre);
                                                app.any_pressed = true;
                                            }
                                        }

                                        if app.any_pressed {
                                            match app.cur_mouse_pos {
                                                None => {}
                                                Some(p) => {
                                                    app.prev_mouse_pos = Some(p);
                                                }
                                            }

                                            let (xr, yr) = cursor_position(((pos.x - offset) as u32, (pos.y+app.scroll_qty - 50.0) as u32), app.window_image_ratio);

                                            app.cur_mouse_pos = Some((xr, yr));

                                            match app.prev_mouse_pos {
                                                None => {}
                                                Some(_p) => {
                                                    let ((x, y), (w, h)) = app.draw_layer.as_ref().unwrap().get_pos_size().unwrap();
                                                    let ((xn, yn), (wn, hn)) = match app.which_shape.as_ref().unwrap() {
                                                        Shape::FilledCircle | Shape::EmptyCircle => {
                                                            get_new_area_circle(
                                                                app.prev_mouse_pos.unwrap(),
                                                                app.cur_mouse_pos.unwrap(),
                                                                (x, y),
                                                                w,
                                                                (app.backup_image_to_save.as_ref().unwrap().width(), app.backup_image_to_save.as_ref().unwrap().height()),
                                                                app.corner.unwrap()
                                                            )
                                                        },
                                                        _ => {
                                                            get_new_area(
                                                                app.prev_mouse_pos.unwrap(),
                                                                app.cur_mouse_pos.unwrap(),
                                                                (x, y),
                                                                (w, h),
                                                                (app.backup_image_to_save.as_ref().unwrap().width(), app.backup_image_to_save.as_ref().unwrap().height()),
                                                                app.corner.unwrap()
                                                            )
                                                        }
                                                    };

                                                    match app.which_shape.unwrap() {
                                                        Shape::FilledRectangle => Image::draw_filled_rectangle(app.draw_layer.as_mut().unwrap(),
                                                                                                               app.rubber_layer.as_mut().unwrap(),
                                                                                                               ((xn + wn / 2) as i32, (yn + hn / 2) as i32),
                                                                                                               (wn as i32, hn as i32), &app.draw_color),
                                                        Shape::EmptyRectangle => Image::draw_empty_rectangle(app.draw_layer.as_mut().unwrap(),
                                                                                                             app.rubber_layer.as_mut().unwrap(),
                                                                                                             ((xn + wn / 2) as i32, (yn + hn / 2) as i32),
                                                                                                             (wn as i32, hn as i32), &app.draw_color, app.pencil_rubber_thickness),
                                                        Shape::FilledCircle => Image::draw_filled_circle(app.draw_layer.as_mut().unwrap(),
                                                                                                         app.rubber_layer.as_mut().unwrap(),
                                                                                                         ((xn + wn / 2) as i32, (yn + hn / 2) as i32),
                                                                                                         wn as i32, &app.draw_color),
                                                        Shape::EmptyCircle => Image::draw_empty_circle(app.draw_layer.as_mut().unwrap(),
                                                                                                       app.rubber_layer.as_mut().unwrap(),
                                                                                                       ((xn + wn / 2) as i32, (yn + hn / 2) as i32),
                                                                                                       wn as i32, &app.draw_color, app.pencil_rubber_thickness),
                                                        Shape::Arrow(dir) => match dir {
                                                            Pointing::Left => Image::draw_filled_left_arrow(app.draw_layer.as_mut().unwrap(),
                                                                                                            app.rubber_layer.as_mut().unwrap(),
                                                                                                            ((xn + wn / 2) as i32, (yn + hn / 2) as i32),
                                                                                                            (wn as i32, hn as i32), &app.draw_color),
                                                            Pointing::Right => Image::draw_filled_right_arrow(app.draw_layer.as_mut().unwrap(),
                                                                                                              app.rubber_layer.as_mut().unwrap(),
                                                                                                              ((xn + wn / 2) as i32, (yn + hn / 2) as i32),
                                                                                                              (wn as i32, hn as i32), &app.draw_color),
                                                            Pointing::Up => Image::draw_filled_up_arrow(app.draw_layer.as_mut().unwrap(),
                                                                                                        app.rubber_layer.as_mut().unwrap(),
                                                                                                        ((xn + wn / 2) as i32, (yn + hn / 2) as i32),
                                                                                                        (wn as i32, hn as i32), &app.draw_color),
                                                            Pointing::Down => Image::draw_filled_down_arrow(app.draw_layer.as_mut().unwrap(),
                                                                                                            app.rubber_layer.as_mut().unwrap(),
                                                                                                            ((xn + wn / 2) as i32, (yn + hn / 2) as i32),
                                                                                                            (wn as i32, hn as i32), &app.draw_color),
                                                        }
                                                    }

                                                    let di = app.draw_layer.as_ref().unwrap().show_shape(app.rubber_layer.as_ref().unwrap());

                                                    app.backup_image = Some(ctx.load_texture(
                                                        "my-image",
                                                        get_image_from_memory(di, 0, 0, 1, 1),
                                                        Default::default()
                                                    ));
                                                }
                                            }

                                            match app.corner.unwrap() {
                                                Corner::UpLeft | Corner::UpRight | Corner::DownLeft | Corner::Centre => {
                                                    let (x, y) = cursor_position(((pos.x - offset) as u32, (pos.y+app.scroll_qty-50.0) as u32), 1.0 / app.window_image_ratio);
                                                    app.anchor_corner = Some(((x as f32, y as f32), app.window_image_ratio));
                                                }
                                                _ => {}
                                            }
                                        }

                                        if ctx.input(|i| i.pointer.any_released()) && app.any_pressed {
                                            app.any_pressed = false;
                                            app.corner = None;
                                            app.prev_mouse_pos = None;
                                            app.cur_mouse_pos = None;
                                        }
                                    }
                                    _ => {}
                                }
                            },
                        }
                    }
                }

                ui.vertical_centered(|ui| {
                        ui.add(egui::Image::new(app.backup_image.as_ref().unwrap(),
                                                app.backup_image.as_ref().unwrap().size_vec2() * app.window_image_ratio));
                });

                ui.horizontal(|ui| {
                    ui.style_mut().visuals.override_text_color = Some(egui::Color32::WHITE);
                    if ui.add(egui::Button::new("OK")).clicked() {
                        if app.draw_layer.is_some() && app.rubber_layer.is_some() {
                            match app.draw_status {
                                DrawStatus::Shape(_) => {
                                    app.backup_image_to_save.as_mut().unwrap().shape_set(app.rubber_layer.take().unwrap(), app.draw_layer.take().unwrap());
                                    let di = app.backup_image_to_save.as_ref().unwrap().show();
                                    app.backup_image = Some(ctx.load_texture(
                                        "my-image",
                                        get_image_from_memory(di, 0, 0, 1, 1),
                                        Default::default()
                                    ));
                                },
                                _ => {}
                            }
                        }

                        app.image = app.backup_image.clone();
                        app.image_to_save = app.backup_image_to_save.clone();

                        app.prev = app.status;
                        app.status = Image;
                        app.draw_status = DrawStatus::Shape(0);

                        if !app.all_images.is_empty(){
                            let _ = app.all_images.remove(app.sel_image);
                            let _ = app.all_images_to_save.remove(app.sel_image);
                            app.all_images_to_save.insert(app.sel_image, app.image_to_save.as_ref().unwrap().clone());
                            app.all_images.insert(app.sel_image, app.image.as_ref().unwrap().clone());
                        }
                    }

                    if ui.add(egui::Button::new("↩")).clicked(){
                        match app.draw_status {
                            DrawStatus::Shape(_) => {
                                app.backup_image_to_save.as_mut().unwrap().shape_set(app.rubber_layer.take().unwrap(), app.draw_layer.take().unwrap());
                                app.draw_status = DrawStatus::default();
                            },
                            _ => {}
                        }
                        let di = app.backup_image_to_save.as_mut().unwrap().undo();
                        app.backup_image = Some(ctx.load_texture(
                            "my-image",
                            get_image_from_memory(di, 0, 0, 1, 1),
                            Default::default()
                        ));
                        match app.draw_status {
                            DrawStatus::Draw => {
                                app.draw_layer = Some(app.backup_image_to_save.as_ref().unwrap().free_hand_draw_init());
                            },
                            DrawStatus::Highlight => {
                                let (rl, dl) = app.backup_image_to_save.as_ref().unwrap().rubber_init(app.last_crop_data);
                                app.rubber_layer = Some(rl);
                                app.draw_layer = Some(dl);
                            },
                            DrawStatus::Rubber => {
                                let (rl, dl) = app.backup_image_to_save.as_ref().unwrap().highlight_init();
                                app.rubber_layer = Some(rl);
                                app.draw_layer = Some(dl);
                            },
                            _ => {}
                        }
                    }

                    if ui.add(egui::Button::new("Back")).clicked() {
                        app.prev = app.status;
                        app.backup_image = app.image.clone();
                        app.backup_image_to_save = app.image_to_save.clone();
                        app.status = Image;
                        if !app.all_images.is_empty(){
                            let _ = app.all_images.remove(app.sel_image);
                            let _ = app.all_images_to_save.remove(app.sel_image);
                            app.all_images_to_save.insert(app.sel_image, app.image_to_save.as_ref().unwrap().clone());
                            app.all_images.insert(app.sel_image, app.image.as_ref().unwrap().clone());
                        }
                    }
                });
            });
    });
}