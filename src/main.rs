//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod hotkey_popup;
mod main_window;
mod screensh;
mod image_proc;
mod cursor_scaling;
mod load_fonts;

use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::fs;
use std::io::Write;
use eframe::egui;
//use eframe::epaint::image;
//use eframe::egui::accesskit::Role::Status;
use global_hotkey::{hotkey::HotKey, GlobalHotKeyEvent, GlobalHotKeyManager};
use keyboard_types::{Code, Modifiers};
use crate::main_window::*;
use crate::hotkey_popup::*;



fn main() -> Result<(), eframe::Error> {
    //let manager = GlobalHotKeyManager::new().unwrap();

    /*let hotkey1: HotKey = "shift+alt+KeyA".parse().unwrap();
    let hotkey4 = HotKey::new(Some(Modifiers::SHIFT | Modifiers::ALT), Code::KeyC);*/

    /*println!("{}", hotkey.id());
    println!("{}", hotkey2.id());
    println!("{}", hotkey3.id());
    println!("{}", hotkey4.id()); */

    /*manager.register(hotkey1).unwrap();
    manager.register(hotkey4).unwrap();*/
    //return Ok(());

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

/*impl Display for Code{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {

    }
}*/

