use std::collections::HashMap;

use global_hotkey::hotkey::Code;
use global_hotkey::hotkey::Modifiers;
use serde::Deserialize;
use serde::Serialize;
#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Trigger {
    pub modifiers: Option<Modifiers>,
    pub code: Code,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Action {
    Key(Option<Modifiers>, Code),
    Smart,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Mapping {
    pub mappings: HashMap<Trigger, Action>,
}

impl Mapping {
    pub fn new() -> Self {
        Mapping {
            mappings: HashMap::new(),
        }
    }

    pub fn add_mapping(&mut self, trigger: Trigger, action: Action) {
        self.mappings.insert(trigger, action);
    }

    pub fn serialize(&self) -> Vec<u8> {
        // use serde serialize
        bincode::serialize(&self).unwrap()
    }

    pub fn deserialize(data: &[u8]) -> Mapping {
        // use serde deserialize
        bincode::deserialize::<Mapping>(data).unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Frame {
    Trigger(Trigger),
    Mapping(Mapping),
}

pub fn modifier2keycode(modifiers: &Modifiers) -> u8 {
    if modifiers == &Modifiers::ALT {
        0b00_000_100
    // Right ALT
    } else if modifiers == &Modifiers::ALT_GRAPH {
        0b00_000_010
    } else if modifiers == &Modifiers::CAPS_LOCK {
        0x1
    } else if modifiers == &Modifiers::CONTROL {
        0x1
    } else {
        0
    }
}

pub fn code2keycode(code: &Code) -> u8 {
    match code {
        Code::Digit0 => KEY_0,
        Code::Digit1 => KEY_1,
        Code::Digit2 => KEY_2,
        Code::Digit3 => KEY_3,
        Code::Digit4 => KEY_4,
        Code::Digit5 => KEY_5,
        Code::Digit6 => KEY_6,
        Code::Digit7 => KEY_7,
        Code::Digit8 => KEY_8,
        Code::Digit9 => KEY_9,
        Code::KeyA => KEY_A,
        Code::KeyB => KEY_B,
        Code::KeyC => KEY_C,
        Code::KeyD => KEY_D,
        Code::KeyE => KEY_E,
        Code::KeyF => KEY_F,
        Code::KeyG => KEY_G,
        Code::KeyH => KEY_H,
        Code::KeyI => KEY_I,
        Code::KeyJ => KEY_J,
        Code::KeyK => KEY_K,
        Code::KeyL => KEY_L,
        Code::KeyM => KEY_M,
        Code::KeyN => KEY_N,
        Code::KeyO => KEY_O,
        Code::KeyP => KEY_P,
        Code::KeyQ => KEY_Q,
        Code::KeyR => KEY_R,
        Code::KeyS => KEY_S,
        Code::KeyT => KEY_T,
        Code::KeyU => KEY_U,
        Code::KeyV => KEY_V,
        Code::KeyW => KEY_W,
        Code::KeyX => KEY_X,
        Code::KeyY => KEY_Y,
        Code::KeyZ => KEY_Z,
        _ => 0,
    }
}
pub const LEFT_CTRL: u8 = 0b00000001;
pub const LEFT_SHIFT: u8 = 0b00000010;
pub const LEFT_ALT: u8 = 0b00000100;
pub const LEFT_GUI: u8 = 0b00001000;
pub const RIGHT_CTRL: u8 = 0b00010000;
pub const RIGHT_SHIFT: u8 = 0b00100000;
pub const RIGHT_ALT: u8 = 0b01000000;
pub const RIGHT_GUI: u8 = 0b10000000;

pub const KEY_A: u8 = 0x4;
pub const KEY_B: u8 = 0x5;
pub const KEY_C: u8 = 0x6;
pub const KEY_D: u8 = 0x7;
pub const KEY_E: u8 = 0x8;
pub const KEY_F: u8 = 0x9;
pub const KEY_G: u8 = 0xa;
pub const KEY_H: u8 = 0xb;
pub const KEY_I: u8 = 0xc;
pub const KEY_J: u8 = 0xd;
pub const KEY_K: u8 = 0xe;
pub const KEY_L: u8 = 0xf;
pub const KEY_M: u8 = 0x10;
pub const KEY_N: u8 = 0x11;
pub const KEY_O: u8 = 0x12;
pub const KEY_P: u8 = 0x13;
pub const KEY_Q: u8 = 0x14;
pub const KEY_R: u8 = 0x15;
pub const KEY_S: u8 = 0x16;
pub const KEY_T: u8 = 0x17;
pub const KEY_U: u8 = 0x18;
pub const KEY_V: u8 = 0x19;
pub const KEY_W: u8 = 0x1a;
pub const KEY_X: u8 = 0x1b;
pub const KEY_Y: u8 = 0x1c;
pub const KEY_Z: u8 = 0x1d;
pub const KEY_1: u8 = 0x1e;
pub const KEY_2: u8 = 0x1f;
pub const KEY_3: u8 = 0x20;
pub const KEY_4: u8 = 0x21;
pub const KEY_5: u8 = 0x22;
pub const KEY_6: u8 = 0x23;
pub const KEY_7: u8 = 0x24;
pub const KEY_8: u8 = 0x25;
pub const KEY_9: u8 = 0x26;
pub const KEY_0: u8 = 0x27;
pub const ENTER: u8 = 0x28;
pub const ESC: u8 = 0x29;
pub const BACKSPACE: u8 = 0x2a;
pub const TAB: u8 = 0x2b;
pub const SPACE: u8 = 0x2c;
pub const KEY_MINUS: u8 = 0x2d;
pub const KEY_EQUAL: u8 = 0x2e;
pub const OPEN_SQUARE_BRACKET: u8 = 0x2f;
pub const CLOSE_SQUARE_BRACKET: u8 = 0x30;
pub const BACKSLASH: u8 = 0x31;
pub const SEMICOLON: u8 = 0x33;
pub const SINGLE_QUOTE: u8 = 0x34; // ' "
pub const BACKTICK: u8 = 0x35; // ` ~
pub const COMMA: u8 = 0x36; // , <
pub const DOT: u8 = 0x37; // . >
pub const SLASH: u8 = 0x38; // / ?
pub const CAPSLOCK: u8 = 0x39;
pub const F1: u8 = 0x3a;
pub const F2: u8 = 0x3b;
pub const F3: u8 = 0x3c;
pub const F4: u8 = 0x3d;
pub const F5: u8 = 0x3e;
pub const F6: u8 = 0x3f;
pub const F7: u8 = 0x40;
pub const F8: u8 = 0x41;
pub const F9: u8 = 0x42;
pub const F10: u8 = 0x43;
pub const F11: u8 = 0x44;
pub const F12: u8 = 0x45;
pub const PRINT_SCREEN: u8 = 0x46;
pub const SCROLL_LOCK: u8 = 0x47;
pub const PAUSE: u8 = 0x48; // Pause Break
pub const DELETE: u8 = 0x4c;
pub const KEY_RIGHT: u8 = 0x4f;
pub const KEY_LEFT: u8 = 0x50;
pub const KEY_DOWN: u8 = 0x51;
pub const KEY_UP: u8 = 0x52;
