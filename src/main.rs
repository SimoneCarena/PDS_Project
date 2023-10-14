#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod hotkey_popup;
mod main_window;
mod screensh;
mod image_proc;
mod cursor_scaling;
mod load_fonts;
mod load_assets;

use eframe::egui;
use crate::main_window::*;

fn main() -> Result<(), eframe::Error> {

    let native_options = eframe::NativeOptions{
        resizable: true,
        min_window_size: Some(egui::Vec2::new(650.0, 300.0)),
        initial_window_size: Some(egui::Vec2::new(650.0, 450.0)),
        icon_data: Some(load_icon("icon.png")),
        ..Default::default()
    };

    eframe::run_native(
        "Screenshot Tool",
        native_options,
        Box::new(|cc| Box::<MyApp>::new(MyApp::new(cc)))
    ).expect("TODO: panic message");

    Ok(())
}

fn load_icon(path: &str) -> eframe::IconData {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    eframe::IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}

