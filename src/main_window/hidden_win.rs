
use eframe::egui;

use crate::main_window::Status::*;

use crate::image_proc::{get_image, load_image_from_memory, get_image_from_memory};

use crate::main_window::{DrawStatus, min_my, MyApp, Pointing, Shape, take_capture};

pub fn hidden_window(app: &mut MyApp, ctx: &egui::Context, frame: &mut eframe::Frame){
    let mut enabled;
    if !app.instant_flag{
        enabled = ctx.input(|i| i.time) - app.disabled_time > app.delay_secs as f64;
    }else{
        enabled = ctx.input(|i| i.time) - app.disabled_time > 0.0;
    }

    if !enabled{
        ctx.request_repaint();
    }else {
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
        frame.set_visible(true);
        app.status = Image;
    }
}