#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

/*mod hotkey_popup;
mod main_window;*/

use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use eframe::egui;
use eframe::egui::scroll_area::ScrollBarVisibility;
use global_hotkey::{hotkey::HotKey, GlobalHotKeyEvent, GlobalHotKeyManager};
use keyboard_types::{Code, Modifiers};
use crate::main_window::MyApp;

#[derive(Clone, Hash)]
pub enum OpName{
    New,
    Save,
    Delay
}

#[derive(Clone, Hash)]
pub struct Operation{
    id: HotKey,
    name: String,
    alt: bool,
    shift: bool,
    ctrl: bool,
    sel_key: Code
}

impl Operation{
    pub fn new(id: HotKey, name: String, alt: bool, shift: bool, ctrl: bool, sel_key: Code) -> Self{
        Operation{id, name, alt, shift, ctrl, sel_key}
    }

    pub fn get_id(&self) -> u32{
        self.id.id()
    }

    pub fn get_name(&self) -> String{
        self.name.clone()
    }

    pub fn id_gen(&self) -> (u32, String, HotKey) {
        let mut hotkey_str = String::new();
        if self.alt {
            hotkey_str.push_str("alt+");
        }
        if self.shift {
            hotkey_str.push_str("shift+");
        }
        if self.ctrl {
            hotkey_str.push_str("ctrl+");
        }
        hotkey_str.push_str((format!("{}", self.sel_key)).as_str());
        let hotkey: HotKey = hotkey_str.clone().as_str().parse().unwrap();
        (hotkey.id(), hotkey_str.clone(), hotkey)
    }

    pub fn get_mut_alt(&mut self) -> &mut bool{
        &mut self.alt
    }
    pub fn get_immut_alt(&self) -> &bool{
        &self.alt
    }

    pub fn get_mut_shift(&mut self) -> &mut bool{
        &mut self.shift
    }
    pub fn get_immut_shift(&self) -> &bool{
        &self.shift
    }

    pub fn get_mut_ctrl(&mut self) -> &mut bool{
        &mut self.ctrl
    }
    pub fn get_immut_ctrl(&self) -> &bool{
        &self.ctrl
    }

    pub fn get_mut_selkey(&mut self) -> &mut Code{
        &mut self.sel_key
    }
    pub fn get_immut_selkey(&self) -> &Code{
        &self.sel_key
    }

    pub fn get_immut_hotkey(&self) -> HotKey {self.id.clone()}

    pub fn parse(str: String) -> Result<Operation, String>{

        if !str.contains("Key"){
            return Err("Non valid string".to_string());
        }

        let mut alt=false;
        let mut shift=false;
        let mut ctrl=false;

        if str.contains("alt"){
            alt = true;
        }
        if str.contains("shift"){
            shift = true;
        }
        if str.contains("ctrl"){
            ctrl = true;
        }

        let hk: HotKey = str.parse().unwrap();

        let key = str.split("+").last().unwrap().to_string();
        let code = string_to_code(&key);

        let op = Operation::new(hk, "".to_string(), alt, shift, ctrl, code);
        Ok(op)
    }
}

impl PartialEq<Self> for Operation {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Operation{}

impl Default for Operation{
    fn default() -> Self {
        Self {
            id: "KeyQ".parse().unwrap(),
            name: String::from("None"),
            alt: false,
            shift: false,
            ctrl: false,
            sel_key: Code::KeyQ
        }
    }
}

pub struct HotKeyPopUp {
    shortcuts: Vec<Operation>
}

impl HotKeyPopUp {

    pub fn new() -> Self{
        HotKeyPopUp{shortcuts: Vec::new()}
    }

    pub fn initialize(shortcuts: Vec<Operation>) -> Self{
        HotKeyPopUp{shortcuts}
    }

    pub fn get_all_shortcuts(&self) -> Vec<Operation>{
        self.shortcuts.clone()
    }

    pub fn get_shortcuts(&mut self, i: usize) -> &mut Operation{
        &mut self.shortcuts[i]
    }

    pub fn shortcuts_replace(&mut self, i: usize, op: Operation) -> Result<(), String> {
        for (index, operation) in self.shortcuts.iter().enumerate() {
            if index == i{
                continue
            }
            if operation.id == op.id{
                return Err("Hotkey già presente".to_string());
            }
        }
        self.shortcuts[i] = op;
        Ok(())
    }

}

impl Default for HotKeyPopUp {
    fn default() -> Self {
        Self {
            shortcuts: vec![
                Operation::new(HotKey::new(Some(Modifiers::SHIFT | Modifiers::ALT), Code::KeyA), "New capture".to_string(), true, true, false, Code::KeyA),
                Operation::new(HotKey::new(Some(Modifiers::SHIFT | Modifiers::ALT), Code::KeyB), "Delay capture".to_string(), true, true, false, Code::KeyB),
                Operation::new(HotKey::new(Some(Modifiers::SHIFT | Modifiers::ALT), Code::KeyC), "Crop capture".to_string(), true, true, false, Code::KeyC),
                Operation::new(HotKey::new(Some(Modifiers::SHIFT | Modifiers::ALT), Code::KeyD), "Draw capture".to_string(), true, true, false, Code::KeyD),
                Operation::new(HotKey::new(Some(Modifiers::SHIFT | Modifiers::ALT), Code::KeyE), "Text capture".to_string(), true, true, false, Code::KeyE),
                Operation::new(HotKey::new(Some(Modifiers::SHIFT | Modifiers::ALT), Code::KeyF), "Copy to clipboard".to_string(), true, true, false, Code::KeyF),
                Operation::new(HotKey::new(Some(Modifiers::SHIFT | Modifiers::ALT), Code::KeyG), "Save capture".to_string(), true, true, false, Code::KeyG)
            ]
        }
    }
}

pub fn string_to_code(s: &str) -> Code{
    let mut ris = Code::KeyA;
    match s{
        "KeyQ" => ris = Code::KeyQ,
        "KeyW" => ris = Code::KeyW,
        "KeyE" => ris = Code::KeyE,
        "KeyR" => ris = Code::KeyR,
        "KeyT" => ris = Code::KeyT,
        "KeyY" => ris = Code::KeyY,
        "KeyU" => ris = Code::KeyU,
        "KeyI" => ris = Code::KeyI,
        "KeyO" => ris = Code::KeyO,
        "KeyP" => ris = Code::KeyP,
        "KeyA" => ris = Code::KeyA,
        "KeyS" => ris = Code::KeyS,
        "KeyD" => ris = Code::KeyD,
        "KeyF" => ris = Code::KeyF,
        "KeyG" => ris = Code::KeyG,
        "KeyH" => ris = Code::KeyH,
        "KeyJ" => ris = Code::KeyJ,
        "KeyK" => ris = Code::KeyK,
        "KeyL" => ris = Code::KeyL,
        "KeyZ" => ris = Code::KeyZ,
        "KeyX" => ris = Code::KeyX,
        "KeyC" => ris = Code::KeyC,
        "KeyV" => ris = Code::KeyV,
        "KeyB" => ris = Code::KeyB,
        "KeyN" => ris = Code::KeyN,
        "KeyM" => ris = Code::KeyM,
        _ => {}
    }
    ris
}
