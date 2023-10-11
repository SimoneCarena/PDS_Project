use eframe::egui;
use crate::main_window::Status::*;
use crate::image_proc::get_image;
use crate::main_window::{MyApp, take_capture};

pub fn hidden_window(app: &mut MyApp, ctx: &egui::Context, frame: &mut eframe::Frame){
    let enabled;
    if !app.instant_flag{
        enabled = ctx.input(|i| i.time) - app.disabled_time > app.delay_secs as f64;
    }else{
        enabled = ctx.input(|i| i.time) - app.disabled_time > 0.0;
    }

    if !app.all_images.is_empty(){
        app.all_images.clear();
        app.all_images_to_save.clear();
    }

    if !enabled{
        ctx.request_repaint();
    }else {
        if app.all_screens{
            for screen in &app.screens{
                match take_capture(screen) {
                    None => {} // eventualmente da gestire
                    Some(im) => {
                        app.all_images.push(
                            ctx.load_texture(
                                "my-image",
                                get_image(".tmp.png", 0, 0, 1, 1),
                                Default::default()
                            )
                        );
                        app.all_images_to_save.push(im);
                    }
                }
            }
            app.sel_image = 0;
            app.image = Some(app.all_images.get(app.sel_image).unwrap().clone());
            app.image_to_save = Some(app.all_images_to_save.get(app.sel_image).unwrap().clone());
        }
        else {
            match take_capture(&app.screens[app.sel_screen]) {
                None => {} // eventualmente da gestire
                Some(im) => {
                    app.image = Some(ctx.load_texture(
                        "my-image",
                        get_image(".tmp.png", 0, 0, 1, 1),
                        Default::default()
                    ));
                    app.image_to_save = Some(im);
                }
            }
        }
        app.all_screens = false;
        frame.set_visible(true);
        app.status = Image;
    }
}