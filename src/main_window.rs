use std::collections::{HashMap, BTreeMap};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use eframe::egui;
use eframe::epaint::TextureHandle;
use eframe::glow::Context;
use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager};
use global_hotkey::hotkey::HotKey;
use image::DynamicImage;
use rusttype::Font;
use crate::hotkey_popup::*;
use crate::main_window::Status::*;
use crate::cursor_scaling::*;
use crate::screensh::{Screen};
use crate::image_proc::blur_area::BlurArea;
use crate::image_proc::colors::{Color};
use crate::image_proc::Image;
use crate::image_proc::extensions::Extensions;
use crate::image_proc::layer::Layer;
use crate::load_assets::load_borders;
use crate::load_fonts::{load_fonts, load_fonts_fallback};
use crate::main_window::crop_win::crop_window;
use crate::main_window::draw_win::draw_window;
use crate::main_window::hidden_win::hidden_window;
use crate::main_window::image_win::image_window;
use crate::main_window::initial_win::initial_window;
use crate::main_window::settings_win::settings_window;
use crate::main_window::text_win::text_window;

pub mod crop_win;
pub mod draw_win;
pub mod text_win;
pub mod initial_win;
pub mod settings_win;
pub mod image_win;
mod hidden_win;

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
    all_images: Vec<TextureHandle>,
    image: Option<TextureHandle>,
    all_images_to_save: Vec<Image>,
    image_to_save: Option<Image>,
    sel_image: usize,
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
    //pointer: egui::PointerState,
    hk_num: usize,
    any_pressed: bool,
    sel_screen: usize,
    all_screens: bool,
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
    borders: Option<HashMap<String, DynamicImage>>,
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
    //shape: bool,
    //shape_pressed: u8,
    draw_status: DrawStatus,
    pencil_rubber_thickness: i32,
    draw_color: Color,
    highlight_color: Color,
    highlight_thickness: i32,
    which_shape: Option<Shape>,
}

impl MyApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut ret = MyApp{
            prev: Status::default(), status: Status::default(),
            hk: HotKeyPopUp::default(), hk_copy: HotKeyPopUp::default(),
            manager_hk: GlobalHotKeyManager::new().unwrap(),
            screens: Screen::get_screens().unwrap(), image: None,
            image_to_save: None, backup_image: None, backup_image_to_save: None,
            all_images: Vec::new(),
            all_images_to_save: Vec::new(),
            sel_image: 0,
            disabled_time: 0.0, instant_flag: true,
            extension: Extensions::PNG,
            extension_copy: Extensions::PNG,
            save_path: std::env::current_dir().unwrap().as_os_str().to_str().unwrap().to_string(),
            save_path_copy: std::env::current_dir().unwrap().as_os_str().to_str().unwrap().to_string(),
            delay_secs: 3u32, delay_secs_cp: 3u32,
            save_name: String::new(),
            clipboard: arboard::Clipboard::new().unwrap(),
            //pointer: egui::PointerState::default(),
            hk_num: 7usize,
            forbidden_hk: vec![false; 7usize],
            any_pressed: false,
            sel_screen: 0usize,
            all_screens: false,
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
            borders: None,
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
            //shape: false,
            //shape_pressed: 0u8,
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

        ret.borders = Some(load_borders().unwrap());

        if !ret.fonts.as_ref().unwrap().contains_key(ret.sel_font.as_ref().unwrap()){
            ret.sel_font = Some(ret.fonts.as_ref().unwrap().iter().nth(0).unwrap().0.clone());
        }

        ret
    }
}

impl eframe::App for MyApp {

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::dark());

        let id: u32;

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

fn take_capture(screen: &Screen) -> Option<Image> {
    match screen.capture(){
        Ok(sh) => {
            sh.save().expect("Failed to save screenshot");
            match Image::open(".tmp.png") {
                Ok(im) => return Some(im),
                Err(_) => {}
            }
            return None
        }
        Err(_) => {
        }
    }
    return None
}

fn min_my(a: f32, b: f32) -> f32{
    if a > b {
        return b;
    }
    a
}

/*fn max_my(a: f32, b: f32) -> f32{
    if a > b {
        return a;
    }
    b
}*/