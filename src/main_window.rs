use std::cmp::min;
use std::collections::{HashMap, BTreeMap};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::mem::forget;
use std::path::Path;
use eframe::egui;
use eframe::egui::scroll_area::ScrollBarVisibility;
use eframe::egui::{UserAttentionType, Vec2};
use egui::Window;
use eframe::epaint::TextureHandle;
use eframe::glow::Context;
use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager};
use global_hotkey::hotkey::HotKey;
use keyboard_types::{Code, Modifiers};
use rusttype::Font;
use crate::hotkey_popup::*;
use crate::main_window::Status::*;
use crate::{image_proc, screensh};
use crate::cursor_scaling::*;
use crate::screensh::{Screen, Screenshot};
use crate::screensh::screensh_errors::ScreenshotError;
use crate::image_proc::{get_image, load_image_from_memory, get_image_from_memory};
use crate::image_proc::blur_area::BlurArea;
use crate::image_proc::colors::{Color, convert_f32_u8, convert_u8_f32};
use crate::image_proc::load_image_from_path;
use crate::image_proc::Image;
use crate::image_proc::extensions::Extensions;
use crate::image_proc::image_errors::ImageManipulationError;
use crate::image_proc::layer::Layer;
use crate::load_fonts::{load_fonts, load_fonts_fallback};


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Status{
    Start,
    Settings,
    Image,
    Hidden,
    Crop,
    Draw,
    Text,
}

impl Default for Status{
    fn default() -> Self {
        Start
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DrawStatus{
    Draw,
    Rubber,
    Highlight,
    Shape(u8)
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Pointing{
    Up,
    Down,
    Left,
    Right
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Shape{
    FilledRectangle,
    EmptyRectangle,
    FilledCircle,
    EmptyCircle,
    Arrow(Pointing)
}

impl Default for DrawStatus{
    fn default() -> Self{
        DrawStatus::Draw
    }
}

pub struct MyApp {
    prev: Status,
    status: Status,
    hk: HotKeyPopUp,
    hk_copy: HotKeyPopUp,
    manager_hk: GlobalHotKeyManager,
    forbidden_hk: Vec<bool>,
    screens: Vec<Screen>,
    image: Option<TextureHandle>,
    image_to_save: Option<Image>,
    backup_image: Option<TextureHandle>,
    backup_image_to_save: Option<Image>,
    disabled_time: f64,
    instant_flag: bool,
    extension: Extensions,
    extension_copy: Extensions,
    save_path: String,
    save_path_copy: String,
    delay_secs: u32,
    delay_secs_cp: u32,
    save_name: String,
    clipboard: arboard::Clipboard,
    pointer: egui::PointerState,
    hk_num: usize,
    any_pressed: bool,
    sel_screen: usize,
    window_image_ratio: f32,
    scroll_qty: f32,
    is_ratio_along_y: bool,
    corner: Option<Corner>,
    bl_ar: Option<BlurArea>,
    prev_mouse_pos: Option<(u32, u32)>,
    cur_mouse_pos: Option<(u32, u32)>,
    anchor_corner: Option<((f32, f32), f32)>,
    draw_layer: Option<Layer>,
    prev_edge: Option<((i32, i32), (i32, i32), (i32, i32))>,
    fonts: Option<BTreeMap<String, Font<'static>>>,
    sel_font: Option<String>,
    sel_font_size: usize,
    sel_color: Color,
    image_text: String,
    is_sel_color: bool,
    dropdown_on: bool,
    rubber: bool,
    highlight: bool,
    rubber_layer: Option<Layer>,
    last_crop_data: Option<((u32, u32), (u32, u32))>,
    shape: bool,
    shape_pressed: u8,
    draw_status: DrawStatus,
    pencil_rubber_thickness: i32,
    draw_color: Color,
    highlight_color: Color,
    highlight_thickness: i32,
    which_shape: Option<Shape>,
}

impl MyApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut ret = MyApp{
            prev: Status::default(), status: Status::default(),
            hk: HotKeyPopUp::default(), hk_copy: HotKeyPopUp::default(),
            manager_hk: GlobalHotKeyManager::new().unwrap(),
            screens: Screen::get_screens().unwrap(), image: None,
            image_to_save: None, backup_image: None, backup_image_to_save: None,
            disabled_time: 0.0, instant_flag: true,
            extension: Extensions::PNG,
            extension_copy: Extensions::PNG,
            save_path: std::env::current_dir().unwrap().as_os_str().to_str().unwrap().to_string(),
            save_path_copy: std::env::current_dir().unwrap().as_os_str().to_str().unwrap().to_string(),
            delay_secs: 3u32, delay_secs_cp: 3u32,
            save_name: String::new(),
            clipboard: arboard::Clipboard::new().unwrap(),
            pointer: egui::PointerState::default(),
            hk_num: 7usize,
            forbidden_hk: vec![false; 7usize],
            any_pressed: false,
            sel_screen: 0usize,
            window_image_ratio: 0.2,  //default
            is_ratio_along_y: true,
            scroll_qty: 0.0,
            corner: None,
            bl_ar: None,
            prev_mouse_pos: None,
            cur_mouse_pos: None,
            anchor_corner: None,
            draw_layer: None,
            prev_edge: None,
            fonts: None,
            sel_font: None,
            sel_font_size: 12usize,
            sel_color: Color::new(0, 0, 0, 1.0),
            image_text: String::from("Insert text here"),
            is_sel_color: false,
            dropdown_on: false,
            rubber: false,
            highlight: false,
            rubber_layer: None,
            last_crop_data: None,
            shape: false,
            shape_pressed: 0u8,
            draw_status: DrawStatus::default(),
            pencil_rubber_thickness: 5,
            draw_color: Color::new(255, 0, 0, 1.0),
            highlight_color: Color::new(255, 255, 0, 0.3),
            highlight_thickness: 5,
            which_shape: None
        };

        match File::open("settings/settings"){
            Ok(f) => {
                let mut vec_op = Vec::new();
                let br = BufReader::new(f);
                for (i, l) in br.lines().enumerate(){
                    let h = l.unwrap();
                    match i {
                        0..=6 => {
                            match parse(h.clone(), i) {
                                Ok(s) => {
                                    vec_op.push(s);
                                    let hh: HotKey = h.parse().unwrap();
                                    ret.manager_hk.register(hh).unwrap();
                                },
                                Err(_) => {}
                            }
                        },
                        7 => {
                            ret.hk = HotKeyPopUp::initialize(vec_op.clone());
                            ret.hk_copy = HotKeyPopUp::initialize(vec_op.clone());
                            ret.delay_secs = h.clone().parse().unwrap();
                            ret.delay_secs_cp = h.clone().parse().unwrap();
                        },
                        8 => {
                            match h.clone().as_str() {
                                "PNG" => {ret.extension = Extensions::PNG; ret.extension_copy = Extensions::PNG},
                                "JPG" => {ret.extension = Extensions::JPG; ret.extension_copy = Extensions::JPG},
                                "GIF" => {ret.extension = Extensions::GIF; ret.extension_copy = Extensions::GIF},
                                _ => {}
                            }
                        },
                        9 => {
                            ret.sel_font = Some(h.clone());
                        },
                        10 => {
                            ret.sel_font_size = h.parse().unwrap();
                        },
                        11 => {
                            let mut iter = h.split_whitespace();
                            ret.sel_color.color[0] = iter.next().unwrap().parse().unwrap();
                            ret.sel_color.color[1] = iter.next().unwrap().parse().unwrap();
                            ret.sel_color.color[2] = iter.next().unwrap().parse().unwrap();
                        },
                        12 => {
                            ret.pencil_rubber_thickness = h.parse().unwrap();
                        },
                        13 => {
                            let mut iter = h.split_whitespace();
                            ret.draw_color.color[0] = iter.next().unwrap().parse().unwrap();
                            ret.draw_color.color[1] = iter.next().unwrap().parse().unwrap();
                            ret.draw_color.color[2] = iter.next().unwrap().parse().unwrap();
                        },
                        14 => {
                            ret.highlight_thickness = h.parse().unwrap();
                        },
                        15 => {
                            let mut iter = h.split_whitespace();
                            ret.highlight_color.color[0] = iter.next().unwrap().parse().unwrap();
                            ret.highlight_color.color[1] = iter.next().unwrap().parse().unwrap();
                            ret.highlight_color.color[2] = iter.next().unwrap().parse().unwrap();
                        },
                        16 => {
                            let path = h.clone();
                            if !Path::new(&path).exists(){
                                ret.save_path = std::env::current_dir().unwrap().as_os_str().to_str().unwrap().to_string();
                                ret.save_path_copy = std::env::current_dir().unwrap().as_os_str().to_str().unwrap().to_string();
                            }
                            else{
                                ret.save_path = path.clone();
                                ret.save_path_copy = path.clone();
                            }
                        },
                        _ => {}
                    }
                }
            }
            Err(_) => {} // non si fa nulla
        }

        /*ret.manager_hk.register(HotKey::new(Some(Modifiers::SHIFT | Modifiers::ALT), Code::KeyA)).unwrap();
        ret.manager_hk.register(HotKey::new(Some(Modifiers::SHIFT | Modifiers::ALT), Code::KeyB)).unwrap();
        ret.manager_hk.register(HotKey::new(Some(Modifiers::SHIFT | Modifiers::ALT), Code::KeyC)).unwrap();
        ret.manager_hk.register(HotKey::new(Some(Modifiers::SHIFT | Modifiers::ALT), Code::KeyD)).unwrap();
        ret.manager_hk.register(HotKey::new(Some(Modifiers::SHIFT | Modifiers::ALT), Code::KeyE)).unwrap();
        ret.manager_hk.register(HotKey::new(Some(Modifiers::SHIFT | Modifiers::ALT), Code::KeyF)).unwrap();
        ret.manager_hk.register(HotKey::new(Some(Modifiers::SHIFT | Modifiers::ALT), Code::KeyG)).unwrap();*/

        ret.fonts = Some(match load_fonts(){
            Ok(x) => {x}
            Err(_) => {
                match load_fonts_fallback(){
                    Ok(y) => {
                        y
                    }
                    Err(_) => {
                        panic!(); //todo()!
                    }
                }
            }
        });

        ret
    }
}

impl eframe::App for MyApp {

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::dark());

        let mut id: u32 = 0;

        if let Ok(event) = GlobalHotKeyEvent::receiver().try_recv() {
            id = event.id;

            for op in &self.hk.get_all_shortcuts(){
                if op.get_id() == id {
                    match op.get_name().as_str(){
                        "New capture" => {
                            frame.set_visible(false);
                            self.disabled_time = ctx.input(|i| i.time);
                            self.prev = self.status.clone();
                            self.instant_flag = true;
                            self.status = Hidden;
                        },
                        "Delay capture" => {
                            frame.set_visible(false);
                            self.disabled_time = ctx.input(|i| i.time);
                            self.prev = self.status.clone();
                            self.instant_flag = false;
                            self.status = Hidden;
                        },
                        "Crop capture" => {
                            if self.image_to_save.is_some() {
                                let w = self.image_to_save.as_ref().unwrap().width();
                                let h = self.image_to_save.as_ref().unwrap().height();
                                let blur = self.image_to_save.as_ref().unwrap().blur_area(0, 0, w, h);
                                self.anchor_corner = Some(((0.0, 0.0), self.window_image_ratio));
                                self.prev_mouse_pos = None;
                                self.cur_mouse_pos = None;
                                self.bl_ar = Some(blur);
                                self.prev = self.status;
                                self.backup_image = self.image.clone();
                                self.backup_image_to_save = self.image_to_save.clone();
                                self.status = Crop;
                            }
                        },
                        "Draw capture" => {
                            if self.image_to_save.is_some() {
                                self.draw_layer = Some(self.image_to_save.as_ref().unwrap().free_hand_draw_init());
                                self.backup_image = self.image.clone();
                                self.backup_image_to_save = self.image_to_save.clone();
                                self.highlight = false;
                                self.rubber = false;
                                self.status = Draw;
                            }
                        },
                        "Text capture" => {
                            if self.image_to_save.is_some() {
                                self.backup_image = self.image.clone();
                                self.backup_image_to_save = self.image_to_save.clone();
                                self.prev = self.status;
                                self.status = Text;
                            }
                        },
                        "Copy to clipboard" => {
                            if self.image_to_save.is_some() {
                                self.image_to_save.as_ref().unwrap().copy_to_clipboard(&mut self.clipboard).unwrap();
                            }
                        },
                        "Save capture" => {
                            if self.image_to_save.is_some() {
                                let mut location = String::from(self.save_path.as_str());
                                if cfg!(target_os = "windows") {
                                    if !self.save_path.ends_with("\\") {
                                        location.push_str("\\");
                                    }
                                    if self.save_name.len() == 0 {
                                        self.image_to_save.as_ref().unwrap().save_as(location.as_str(), "", self.extension).unwrap();
                                    } else {
                                        self.image_to_save.as_ref().unwrap().save_as(location.as_str(), self.save_name.as_str(), self.extension).unwrap();
                                    }
                                    self.save_name = String::new();
                                } else if cfg!(target_os = "macos") || cfg!(target_os = "linux") {
                                    if !self.save_path.ends_with("/") {
                                        location.push_str("/");
                                    }
                                    if self.save_name.len() == 0 {
                                        self.image_to_save.as_ref().unwrap().save_as(location.as_str(), "", self.extension).unwrap();
                                    } else {
                                        self.image_to_save.as_ref().unwrap().save_as(location.as_str(), self.save_name.as_str(), self.extension).unwrap();
                                    }
                                    self.save_name = String::new();
                                } else {
                                    panic!(); // da gestire
                                }
                            }
                        },
                        _ => {}
                    }

                    break;
                }
            }
        }

        match self.status {
            Start => {
                initial_window(self, ctx, frame);
            }
            Settings => {
                settings_window(self, ctx, frame);
            }
            Image => {
                image_window(self, ctx, frame);
            }
            Hidden => {
                hidden_window(self, ctx, frame);
            }
            Crop => {
                crop_window(self, ctx, frame);
            },
            Draw => {
                draw_window(self, ctx, frame);
            },
            Text => {
                text_window(self, ctx, frame);
            }
        }
    }

    fn on_exit(&mut self, _gl: Option<&Context>) {
        match File::create("settings/settings"){
            Ok(mut f) => {
                for el in self.hk.get_all_shortcuts(){
                    let (_,mut b,_) = el.id_gen();
                    b.push_str("\n");
                    f.write_all(b.as_bytes()).unwrap();
                }
                f.write_all(format!("{}\n", self.delay_secs).as_bytes()).unwrap();
                f.write_all(format!("{}\n", self.extension).as_bytes()).unwrap();
                f.write_all(format!("{}\n", self.sel_font.as_ref().unwrap()).as_bytes()).unwrap();
                f.write_all(format!("{}\n", self.sel_font_size).as_bytes()).unwrap();
                f.write_all(format!("{} {} {}\n",
                                    self.sel_color.color.0[0],
                                    self.sel_color.color.0[1],
                                    self.sel_color.color.0[2]
                ).as_bytes()).unwrap();
                f.write_all(format!("{}\n", self.pencil_rubber_thickness).as_bytes()).unwrap();
                f.write_all(format!("{} {} {}\n",
                                    self.draw_color.color.0[0],
                                    self.draw_color.color.0[1],
                                    self.draw_color.color.0[2]
                ).as_bytes()).unwrap();
                f.write_all(format!("{}\n", self.highlight_thickness).as_bytes()).unwrap();
                f.write_all(format!("{} {} {}\n",
                                    self.highlight_color.color.0[0],
                                    self.highlight_color.color.0[1],
                                    self.highlight_color.color.0[2]
                ).as_bytes()).unwrap();
                f.write_all(format!("{}\n", self.save_path).as_bytes()).unwrap();
            }
            Err(_) => {} // non si fa nulla
        }
    }
}

fn initial_window(app: &mut MyApp, ctx: &egui::Context, frame: &mut eframe::Frame){
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

        ui.vertical_centered(|ui|{
            ui.add(egui::TextEdit::singleline(&mut "Press Take to capture a new screenshot"));
        });
    });
}

fn image_window(app: &mut MyApp, ctx: &egui::Context, frame: &mut eframe::Frame){
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
        //println!("{:?}  {:?}", window_size.height()/screen_size.x, window_size.width()/screen_size.y);
        //println!("{ratio}");
        //ratio = 0.29;
        //println!("{:?}", app.image.as_ref().unwrap().size_vec2());
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

fn hidden_window(app: &mut MyApp, ctx: &egui::Context, frame: &mut eframe::Frame){
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

fn crop_window(app: &mut MyApp, ctx: &egui::Context, frame: &mut eframe::Frame){
    //pulsanti
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
                        Some(p) => {
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
                            let di = app.bl_ar.as_ref().unwrap().show();

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

        ui.vertical_centered(|ui| {
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

            }

            if ui.add(egui::Button::new("Back")).clicked(){
                app.prev = app.status;
                app.status = Image;
                app.bl_ar = None;
                app.image_to_save = app.backup_image_to_save.clone();
                app.image = app.backup_image.clone()
            }
        });


    });

}

fn text_window(app: &mut MyApp, ctx: &egui::Context, frame: &mut eframe::Frame){
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
                    if ui.add(egui::Button::new("Canc")).clicked() {
                        app.any_pressed = false;
                        app.backup_image = app.image.clone();
                        app.backup_image_to_save = app.image_to_save.clone();
                    }
                });
            });
    });
}

fn draw_window(app: &mut MyApp, ctx: &egui::Context, frame: &mut eframe::Frame){

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
                                    app.image_to_save.as_mut().unwrap().shape_set(app.rubber_layer.take().unwrap(), app.draw_layer.take().unwrap());
                                    let di = app.image_to_save.as_ref().unwrap().show();
                                    app.image = Some(ctx.load_texture(
                                        "my-image",
                                        get_image_from_memory(di, 0, 0, 1, 1),
                                        Default::default()
                                    ));
                                }
                                _ => {}
                            }
                        }
                        app.draw_layer = Some(app.image_to_save.as_ref().unwrap().free_hand_draw_init());
                        app.draw_status = DrawStatus::Draw;
                    }
                    if ui.button("🗑 Erase").on_hover_text("Erase annotations").clicked() {
                        app.rubber = true;
                        if app.draw_layer.is_some() && app.rubber_layer.is_some() {
                            match app.draw_status {
                                DrawStatus::Shape(1) => {
                                    app.image_to_save.as_mut().unwrap().shape_set(app.rubber_layer.take().unwrap(), app.draw_layer.take().unwrap());
                                    let di = app.image_to_save.as_ref().unwrap().show();
                                    app.image = Some(ctx.load_texture(
                                        "my-image",
                                        get_image_from_memory(di, 0, 0, 1, 1),
                                        Default::default()
                                    ));
                                }
                                _ => {}
                            }
                        }

                        let (rl, dl) = app.image_to_save.as_ref().unwrap().rubber_init(app.last_crop_data);
                        app.rubber_layer = Some(rl);
                        app.draw_layer = Some(dl);
                        app.draw_status = DrawStatus::Rubber;
                    }
                    if ui.button("📌 Highlight").on_hover_text("Activate highlighter").clicked() {
                        app.rubber = false;
                        if app.draw_layer.is_some() && app.rubber_layer.is_some() {
                            match app.draw_status {
                                DrawStatus::Shape(1) => {
                                    app.image_to_save.as_mut().unwrap().shape_set(app.rubber_layer.take().unwrap(), app.draw_layer.take().unwrap());
                                    let di = app.image_to_save.as_ref().unwrap().show();
                                    app.image = Some(ctx.load_texture(
                                        "my-image",
                                        get_image_from_memory(di, 0, 0, 1, 1),
                                        Default::default()
                                    ));
                                }
                                _ => {}
                            }
                        }
                        let (rl, dl) = app.image_to_save.as_ref().unwrap().highlight_init();
                        app.rubber_layer = Some(rl);
                        app.draw_layer = Some(dl);
                        app.draw_status = DrawStatus::Highlight;
                    }

                    match app.draw_status{
                        DrawStatus::Draw | DrawStatus::Rubber => {
                            ui.add(egui::Slider::new(&mut app.pencil_rubber_thickness, 1..=40).text("Trait size"));
                        }
                        DrawStatus::Highlight => {
                            ui.add(egui::Slider::new(&mut app.highlight_thickness, 1..=40).text("Trait size"));
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
                                    app.image_to_save.as_mut().unwrap().shape_set(app.rubber_layer.take().unwrap(), app.draw_layer.take().unwrap());
                                    let di = app.image_to_save.as_ref().unwrap().show();
                                    app.image = Some(ctx.load_texture(
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
                                    app.image_to_save.as_mut().unwrap().shape_set(app.rubber_layer.take().unwrap(), app.draw_layer.take().unwrap());
                                    let di = app.image_to_save.as_ref().unwrap().show();
                                    app.image = Some(ctx.load_texture(
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

                    if ui.button("⚫").on_hover_text("Filled circle").clicked() {
                        if app.draw_layer.is_some() && app.rubber_layer.is_some() {
                            match app.draw_status {
                                DrawStatus::Shape(1) => {
                                    app.image_to_save.as_mut().unwrap().shape_set(app.rubber_layer.take().unwrap(), app.draw_layer.take().unwrap());
                                    let di = app.image_to_save.as_ref().unwrap().show();
                                    app.image = Some(ctx.load_texture(
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
                                    app.image_to_save.as_mut().unwrap().shape_set(app.rubber_layer.take().unwrap(), app.draw_layer.take().unwrap());
                                    let di = app.image_to_save.as_ref().unwrap().show();
                                    app.image = Some(ctx.load_texture(
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
                                    app.image_to_save.as_mut().unwrap().shape_set(app.rubber_layer.take().unwrap(), app.draw_layer.take().unwrap());
                                    let di = app.image_to_save.as_ref().unwrap().show();
                                    app.image = Some(ctx.load_texture(
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
                                    app.image_to_save.as_mut().unwrap().shape_set(app.rubber_layer.take().unwrap(), app.draw_layer.take().unwrap());
                                    let di = app.image_to_save.as_ref().unwrap().show();
                                    app.image = Some(ctx.load_texture(
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
                                    app.image_to_save.as_mut().unwrap().shape_set(app.rubber_layer.take().unwrap(), app.draw_layer.take().unwrap());
                                    let di = app.image_to_save.as_ref().unwrap().show();
                                    app.image = Some(ctx.load_texture(
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
                                    app.image_to_save.as_mut().unwrap().shape_set(app.rubber_layer.take().unwrap(), app.draw_layer.take().unwrap());
                                    let di = app.image_to_save.as_ref().unwrap().show();
                                    app.image = Some(ctx.load_texture(
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

                let mut di;
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

                        //if pos.x - offset > 0.0 && pos.x - offset < image_size.x * app.window_image_ratio
                        //&& pos.y > 25.0 && pos.y < (image_size.y * app.window_image_ratio + 25.0) && !app.any_pressed && !app.dropdown_on {
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

                                    let mut di;
                                    if app.any_pressed {
                                        match app.draw_status {
                                            DrawStatus::Draw => {
                                                app.prev_edge = Some(Image::draw_point(app.draw_layer.as_mut().unwrap(), app.prev_edge.clone(), (cur.0 as i32, cur.1 as i32), app.pencil_rubber_thickness, &app.draw_color));
                                                di = app.draw_layer.as_ref().unwrap().show();
                                                app.image = Some(ctx.load_texture("my-image", get_image_from_memory(di, 0, 0, 1, 1), Default::default()));
                                            },
                                            DrawStatus::Rubber => {
                                                app.prev_edge = Some(Image::rubber(app.draw_layer.as_mut().unwrap(), app.prev_edge.clone(), (cur.0 as i32, cur.1 as i32), app.pencil_rubber_thickness*2));
                                                di = app.draw_layer.as_ref().unwrap().show_rubber(app.rubber_layer.as_ref().unwrap());
                                                app.image = Some(ctx.load_texture("my-image", get_image_from_memory(di, 0, 0, 1, 1), Default::default()));
                                            },
                                            DrawStatus::Highlight => {
                                                app.prev_edge = Some(Image::highlight(app.draw_layer.as_mut().unwrap(), app.prev_edge.clone(), (cur.0 as i32, cur.1 as i32), app.pencil_rubber_thickness, &image_proc::colors::Color::new(app.draw_color.color[0], app.draw_color.color[1], app.draw_color.color[2], 0.3)));
                                                di = app.draw_layer.as_ref().unwrap().show_higlight(app.rubber_layer.as_ref().unwrap());
                                                app.image = Some(ctx.load_texture("my-image", get_image_from_memory(di, 0, 0, 1, 1), Default::default()));
                                            },
                                            _ => {}
                                        }
                                    }

                                    if ctx.input(|i| i.pointer.any_released()) && app.any_pressed {
                                        app.any_pressed = false;
                                        match app.draw_status {
                                            DrawStatus::Draw => {
                                                app.image_to_save.as_mut().unwrap().free_hand_draw_set(app.draw_layer.take().unwrap(), app.prev_edge.unwrap().clone().2, app.pencil_rubber_thickness, &app.draw_color);
                                                app.draw_layer = Some(app.image_to_save.as_ref().unwrap().free_hand_draw_init());
                                            },
                                            DrawStatus::Rubber => {
                                                app.image_to_save.as_mut().unwrap().rubber_set(app.draw_layer.take().unwrap(), app.rubber_layer.as_ref().unwrap(), app.prev_edge.unwrap().clone().2, app.pencil_rubber_thickness*2);
                                                let (rl, dl) = app.image_to_save.as_ref().unwrap().rubber_init(app.last_crop_data);
                                                app.rubber_layer = Some(rl);
                                                app.draw_layer = Some(dl);
                                            },
                                            DrawStatus::Highlight => {
                                                app.image_to_save.as_mut().unwrap().highlight_set(app.draw_layer.take().unwrap(), app.rubber_layer.as_ref().unwrap(), app.prev_edge.unwrap().clone().2, app.highlight_thickness, &app.highlight_color);
                                                let (rl, dl) = app.image_to_save.as_ref().unwrap().highlight_init();
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
                                            //println!("Dentro");
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

                                                    let (rl, dl) = app.image_to_save.as_ref().unwrap().shape_init(start, (300, 200));
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

                                                    //Image::draw_filled_rectangle(app.draw_layer.as_mut().unwrap(), app.rubber_layer.as_mut().unwrap(), (start.0 as i32, start.1 as i32), (300, 200), &app.draw_color);
                                                    //Image::draw_filled_circle(app.draw_layer.as_mut().unwrap(), app.rubber_layer.as_mut().unwrap(), (start.0 as i32, start.1 as i32), 400, &app.draw_color);
                                                    di = app.draw_layer.as_ref().unwrap().show_shape(app.rubber_layer.as_ref().unwrap());
                                                    app.image = Some(
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
                                        //println!("OOOOOO");
                                        let ((x, y), (w, h)) = app.draw_layer.as_ref().unwrap().get_pos_size().unwrap();
                                        let upleft = (x, y);
                                        let upright = (x + w, y);
                                        let downleft = (x, y + h);
                                        let downright = (x + w, y + h);

                                        //println!("{:?}", (x, y));

                                        let c1 = cursor_position(upleft, 1.0 / app.window_image_ratio);
                                        let c1 = (c1.0 as f32, c1.1 as f32);
                                        let c2 = cursor_position(upright, 1.0 / app.window_image_ratio);
                                        let c2 = (c2.0 as f32, c2.1 as f32);
                                        let c3 = cursor_position(downleft, 1.0 / app.window_image_ratio);
                                        let c3 = (c3.0 as f32, c3.1 as f32);
                                        let c4 = cursor_position(downright, 1.0 / app.window_image_ratio);
                                        let c4 = (c4.0 as f32, c4.1 as f32);

                                        //println!("{:?} {}", pos, offset);
                                        //println!("{:?} {:?} {:?} {:?}", c1, c2, c3, c4);

                                        if (pos.x - offset > c1.0 -10.0 && pos.x - offset < c1.0 + 10.0) && (pos.y+app.scroll_qty - 50.0 > c1.1-10.0 && pos.y+app.scroll_qty - 50.0 < c1.1 + 10.0) {
                                            //println!("Angolo!!");
                                            if ctx.input(|i| i.pointer.any_pressed()) {
                                                app.any_pressed = true;
                                                app.corner = Some(Corner::UpLeft);
                                                //println!("pressed");
                                            }
                                        }
                                        //basso a sx
                                        else if (pos.x - offset > c3.0 -10.0 && pos.x - offset < c3.0 + 10.0) && ((pos.y+app.scroll_qty - 50.0 > c3.1 - 10.0) && (pos.y+app.scroll_qty - 50.0 < c3.1 + 10.0)) {
                                            //println!("Angolo!!");
                                            if ctx.input(|i| i.pointer.any_pressed()) {
                                                app.any_pressed = true;
                                                app.corner = Some(Corner::DownLeft);
                                                //println!("pressed");
                                            }
                                        }
                                        //alto a dx
                                        else if ((pos.x - offset > c2.0 - 10.0) && (pos.x - offset < c2.0 + 10.0)) && (pos.y+app.scroll_qty - 50.0 > c2.1 -10.0 && pos.y+app.scroll_qty - 50.0 < c2.1 + 10.0) {
                                            //println!("Angolo!!");
                                            if ctx.input(|i| i.pointer.any_pressed()) {
                                                app.corner = Some(Corner::UpRight);
                                                app.any_pressed = true;
                                                //println!("pressed");
                                            }
                                        }
                                        //basso a dx
                                        else if ((pos.x - offset > c4.0 - 10.0) && (pos.x - offset < c4.0 + 10.0)) && ((pos.y+app.scroll_qty - 50.0 > c4.1 - 10.0) && (pos.y+app.scroll_qty - 50.0 < c4.1 + 10.0)) {
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

                                            let (xr, yr) = cursor_position(((pos.x - offset) as u32, (pos.y+app.scroll_qty - 50.0) as u32), app.window_image_ratio);

                                            app.cur_mouse_pos = Some((xr, yr));

                                            match app.prev_mouse_pos {
                                                None => {}
                                                Some(p) => {
                                                    let ((x, y), (w, h)) = app.draw_layer.as_ref().unwrap().get_pos_size().unwrap();  //app.bl_ar.as_ref().unwrap().get_crop_data();
                                                    //println!("{:?} {:?}", app.prev_mouse_pos.unwrap(), app.cur_mouse_pos.unwrap());
                                                    let ((xn, yn), (wn, hn)) = match app.which_shape.as_ref().unwrap() {
                                                        Shape::FilledCircle | Shape::EmptyCircle => {
                                                            get_new_area_circle(
                                                                app.prev_mouse_pos.unwrap(),
                                                                app.cur_mouse_pos.unwrap(), 
                                                                (x, y), 
                                                                w, 
                                                                (app.image_to_save.as_ref().unwrap().width(), app.image_to_save.as_ref().unwrap().height()), 
                                                                app.corner.unwrap()
                                                            )
                                                        },
                                                        _ => {
                                                            get_new_area(
                                                                app.prev_mouse_pos.unwrap(),
                                                                app.cur_mouse_pos.unwrap(),
                                                                (x, y),
                                                                (w, h),
                                                                (app.image_to_save.as_ref().unwrap().width(), app.image_to_save.as_ref().unwrap().height()),
                                                                app.corner.unwrap()
                                                            )
                                                        }
                                                    };

                                                    /*let ((xn, yn), (wn, hn)) = get_new_area_circle(
                                                    app.prev_mouse_pos.unwrap(),
                                                    app.cur_mouse_pos.unwrap(),
                                                    (x, y),
                                                    w,
                                                    (app.image_to_save.as_ref().unwrap().width(), app.image_to_save.as_ref().unwrap().height()),
                                                    app.corner.unwrap()
                                                );*/

                                                    match app.which_shape.unwrap() {
                                                        Shape::FilledRectangle => Image::draw_filled_rectangle(app.draw_layer.as_mut().unwrap(),
                                                                                                               app.rubber_layer.as_mut().unwrap(),
                                                                                                               ((xn + wn / 2) as i32, (yn + hn / 2) as i32),
                                                                                                               (wn as i32, hn as i32), &app.draw_color
                                                        ),
                                                        Shape::EmptyRectangle => Image::draw_empty_rectangle(app.draw_layer.as_mut().unwrap(),
                                                                                                             app.rubber_layer.as_mut().unwrap(),
                                                                                                             ((xn + wn / 2) as i32, (yn + hn / 2) as i32),
                                                                                                             (wn as i32, hn as i32), &app.draw_color, app.pencil_rubber_thickness
                                                        ),
                                                        Shape::FilledCircle => Image::draw_filled_circle(app.draw_layer.as_mut().unwrap(),
                                                                                                         app.rubber_layer.as_mut().unwrap(),
                                                                                                         ((xn + wn / 2) as i32, (yn + hn / 2) as i32),
                                                                                                         wn as i32, &app.draw_color
                                                        ),
                                                        Shape::EmptyCircle => Image::draw_empty_circle(app.draw_layer.as_mut().unwrap(),
                                                                                                       app.rubber_layer.as_mut().unwrap(),
                                                                                                       ((xn + wn / 2) as i32, (yn + hn / 2) as i32),
                                                                                                       wn as i32, &app.draw_color, app.pencil_rubber_thickness
                                                        ),
                                                        Shape::Arrow(dir) => match dir {
                                                            Pointing::Left => Image::draw_filled_left_arrow(app.draw_layer.as_mut().unwrap(),
                                                                                                            app.rubber_layer.as_mut().unwrap(),
                                                                                                            ((xn + wn / 2) as i32, (yn + hn / 2) as i32),
                                                                                                            (wn as i32, hn as i32), &app.draw_color
                                                            ),
                                                            Pointing::Right => Image::draw_filled_right_arrow(app.draw_layer.as_mut().unwrap(),
                                                                                                              app.rubber_layer.as_mut().unwrap(),
                                                                                                              ((xn + wn / 2) as i32, (yn + hn / 2) as i32),
                                                                                                              (wn as i32, hn as i32), &app.draw_color
                                                            ),
                                                            Pointing::Up => Image::draw_filled_up_arrow(app.draw_layer.as_mut().unwrap(),
                                                                                                        app.rubber_layer.as_mut().unwrap(),
                                                                                                        ((xn + wn / 2) as i32, (yn + hn / 2) as i32),
                                                                                                        (wn as i32, hn as i32), &app.draw_color
                                                            ),
                                                            Pointing::Down => Image::draw_filled_down_arrow(app.draw_layer.as_mut().unwrap(),
                                                                                                            app.rubber_layer.as_mut().unwrap(),
                                                                                                            ((xn + wn / 2) as i32, (yn + hn / 2) as i32),
                                                                                                            (wn as i32, hn as i32), &app.draw_color
                                                            ),
                                                        }
                                                    }

                                                    /*Image::draw_filled_circle(app.draw_layer.as_mut().unwrap(),
                                                                          app.rubber_layer.as_mut().unwrap(),
                                                                          ((xn + wn / 2) as i32, (yn + hn / 2) as i32),
                                                                          wn as i32, &app.draw_color);*/

                                                    let di = app.draw_layer.as_ref().unwrap().show_shape(app.rubber_layer.as_ref().unwrap());    //app.bl_ar.as_ref().unwrap().show();

                                                    app.image = Some(ctx.load_texture(
                                                        "my-image",
                                                        get_image_from_memory(di, 0, 0, 1, 1),
                                                        Default::default()
                                                    ));
                                                }
                                            }

                                            match app.corner.unwrap() {
                                                Corner::UpLeft | Corner::UpRight | Corner::DownLeft => {
                                                    let (x, y) = cursor_position(((pos.x - offset) as u32, (pos.y+app.scroll_qty-50.0) as u32), 1.0 / app.window_image_ratio);
                                                    app.anchor_corner = Some(((x as f32, y as f32), app.window_image_ratio));
                                                }
                                                _ => {} //inutile
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
                        //}
                    }
                }

                ui.vertical_centered(|ui| {
                    ui.add(egui::Image::new(app.image.as_ref().unwrap(), app.image.as_ref().unwrap().size_vec2() * app.window_image_ratio));
                });

                ui.horizontal(|ui| {
                    ui.style_mut().visuals.override_text_color = Some(egui::Color32::WHITE);
                    if ui.add(egui::Button::new("OK")).clicked() {
                        if app.draw_layer.is_some() && app.rubber_layer.is_some() {
                            match app.draw_status {
                                DrawStatus::Shape(_) => {
                                    app.image_to_save.as_mut().unwrap().shape_set(app.rubber_layer.take().unwrap(), app.draw_layer.take().unwrap());
                                    let di = app.image_to_save.as_ref().unwrap().show();
                                    app.image = Some(ctx.load_texture(
                                        "my-image",
                                        get_image_from_memory(di, 0, 0, 1, 1),
                                        Default::default()
                                    ));
                                },
                                _ => {}
                            }
                        }

                        //app.image_to_save.as_mut().unwrap().shape_set(app.rubber_layer.take().unwrap(), app.draw_layer.take().unwrap());
                        app.prev = app.status;
                        app.status = Image;
                        app.draw_status = DrawStatus::Shape(0);
                    }
                    if ui.add(egui::Button::new("Back")).clicked() {
                        app.prev = app.status;
                        app.image = app.backup_image.clone();
                        app.image_to_save = app.backup_image_to_save.clone();
                        app.status = Image;
                    }
                });
            });
    });

}

fn settings_window(app: &mut MyApp, ctx: &egui::Context, frame: &mut eframe::Frame){
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
                        let alt_label = ui.label("ALT: ");
                        ui.checkbox(app.hk_copy.get_shortcuts(i).get_mut_alt(), "")
                            .labelled_by(alt_label.id);
                        let shift_label = ui.label("SHIFT: ");
                        ui.checkbox(app.hk_copy.get_shortcuts(i).get_mut_shift(), "")
                            .labelled_by(shift_label.id);
                        let control_label = ui.label("CONTROL: ");
                        ui.checkbox(app.hk_copy.get_shortcuts(i).get_mut_ctrl(), "")
                            .labelled_by(control_label.id);
                    });

                    egui::ComboBox::from_label(format!("Select the KEY-CODE {}!", i))
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

                    let (mut id, mut str, mut hotk) = app.hk_copy.get_shortcuts(i).id_gen();

                    if app.forbidden_hk[i]{
                        ui.scope(|ui|{
                            ui.style_mut().visuals.override_text_color = Some(egui::Color32::LIGHT_RED);
                            ui.label("Combination already in use; please select another one");
                        });
                    }

                    if i==1{
                        egui::ComboBox::from_label(format!("Select Capture Delay!",))
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
                }

                ui.heading("Save Extension");
                egui::ComboBox::from_label(format!("Select the Save EXTENSION!"))
                    .selected_text(format!("{:?}", app.extension_copy))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut app.extension_copy, Extensions::PNG, "PNG");
                        ui.selectable_value(&mut app.extension_copy, Extensions::JPG, "JPG");
                        ui.selectable_value(&mut app.extension_copy, Extensions::GIF, "GIF");
                    });

                if ui.add(egui::Button::new("OK")).clicked() {
                    app.extension = app.extension_copy;
                }

                ui.heading("Save Directory");
                ui.scope(|ui|{
                    //ui.style_mut().visuals.widgets.hovered.bg_stroke.color = egui::Color32::WHITE;
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

fn take_capture(screen: &Screen) -> Option<Image> {
    match screen.capture(){
        Ok(sh) => {
            sh.save();
            match Image::open(".tmp.png") {
                Ok(im) => return Some(im),
                Err(_) => return None
            }
        }
        Err(_) => {
            return None
        }
    }
}

fn min_my(a: f32, b: f32) -> f32{
    if a > b {
        return b;
    }
    a
}

fn max_my(a: f32, b: f32) -> f32{
    if a > b {
        return a;
    }
    b
}