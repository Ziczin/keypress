use crate::{Key, KeyEvent};
use evdev::{Device, InputEvent, KeyCode};
use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::os::unix::fs::FileTypeExt;
use std::path::Path;

pub fn wait_key_with_mods() -> io::Result<KeyEvent> {
    let device = match find_keyboard() {
        Some(Ok(dev)) => dev,
        _ => return super::crossterm_fallback::wait_key_with_mods(),
    };

    let mut device = device;
    let mut shift = false;
    let mut ctrl = false;
    let mut alt = false;
    let mut pressed = HashSet::new();

    loop {
        let events = device.fetch_events()?;
        for ev in events {
            if ev.event_type() != evdev::EventType::KEY {
                continue;
            }
            let code = ev.code();
            let value = ev.value();
            let key = map_evdev_key(code);

            match value {
                1 => {
                    match key {
                        Key::ShiftLeft | Key::ShiftRight => shift = true,
                        Key::ControlLeft | Key::ControlRight => ctrl = true,
                        Key::AltLeft | Key::AltRight => alt = true,
                        _ => {}
                    }
                    pressed.insert(code);
                    return Ok(KeyEvent { key, shift, ctrl, alt });
                }
                0 => {
                    match key {
                        Key::ShiftLeft | Key::ShiftRight => shift = false,
                        Key::ControlLeft | Key::ControlRight => ctrl = false,
                        Key::AltLeft | Key::AltRight => alt = false,
                        _ => {}
                    }
                    pressed.remove(&code);
                }
                _ => {}
            }
        }
    }
}

fn find_keyboard() -> Option<io::Result<Device>> {
    for entry in std::fs::read_dir("/dev/input/by-path").ok()? {
        let entry = entry.ok()?;
        let path = entry.path();
        if path.file_name().and_then(|s| s.to_str()).map(|s| s.contains("-kbd")).unwrap_or(false) {
            return Some(Device::open(&path));
        }
    }
    for i in 0..32 {
        let path = format!("/dev/input/event{}", i);
        if Path::new(&path).exists() {
            if let Ok(mut dev) = Device::open(&path) {
                if dev.supported_keys().map(|keys| keys.contains(KeyCode::KEY_A)).unwrap_or(false) {
                    return Some(Ok(dev));
                }
            }
        }
    }
    None
}

fn map_evdev_key(key: KeyCode) -> Key {
    use evdev::KeyCode::*;
    match key {
        KEY_ESC => Key::Escape,
        KEY_F1 => Key::F1,
        KEY_F2 => Key::F2,
        KEY_F3 => Key::F3,
        KEY_F4 => Key::F4,
        KEY_F5 => Key::F5,
        KEY_F6 => Key::F6,
        KEY_F7 => Key::F7,
        KEY_F8 => Key::F8,
        KEY_F9 => Key::F9,
        KEY_F10 => Key::F10,
        KEY_F11 => Key::F11,
        KEY_F12 => Key::F12,
        KEY_GRAVE => Key::Grave,
        KEY_1 => Key::Digit1,
        KEY_2 => Key::Digit2,
        KEY_3 => Key::Digit3,
        KEY_4 => Key::Digit4,
        KEY_5 => Key::Digit5,
        KEY_6 => Key::Digit6,
        KEY_7 => Key::Digit7,
        KEY_8 => Key::Digit8,
        KEY_9 => Key::Digit9,
        KEY_0 => Key::Digit0,
        KEY_MINUS => Key::Minus,
        KEY_EQUAL => Key::Equal,
        KEY_BACKSPACE => Key::Backspace,
        KEY_TAB => Key::Tab,
        KEY_Q => Key::KeyQ,
        KEY_W => Key::KeyW,
        KEY_E => Key::KeyE,
        KEY_R => Key::KeyR,
        KEY_T => Key::KeyT,
        KEY_Y => Key::KeyY,
        KEY_U => Key::KeyU,
        KEY_I => Key::KeyI,
        KEY_O => Key::KeyO,
        KEY_P => Key::KeyP,
        KEY_LEFTBRACE => Key::LeftBracket,
        KEY_RIGHTBRACE => Key::RightBracket,
        KEY_BACKSLASH => Key::Backslash,
        KEY_CAPSLOCK => Key::CapsLock,
        KEY_A => Key::KeyA,
        KEY_S => Key::KeyS,
        KEY_D => Key::KeyD,
        KEY_F => Key::KeyF,
        KEY_G => Key::KeyG,
        KEY_H => Key::KeyH,
        KEY_J => Key::KeyJ,
        KEY_K => Key::KeyK,
        KEY_L => Key::KeyL,
        KEY_SEMICOLON => Key::Semicolon,
        KEY_APOSTROPHE => Key::Quote,
        KEY_ENTER => Key::Enter,
        KEY_LEFTSHIFT => Key::ShiftLeft,
        KEY_Z => Key::KeyZ,
        KEY_X => Key::KeyX,
        KEY_C => Key::KeyC,
        KEY_V => Key::KeyV,
        KEY_B => Key::KeyB,
        KEY_N => Key::KeyN,
        KEY_M => Key::KeyM,
        KEY_COMMA => Key::Comma,
        KEY_DOT => Key::Period,
        KEY_SLASH => Key::Slash,
        KEY_RIGHTSHIFT => Key::ShiftRight,
        KEY_LEFTCTRL => Key::ControlLeft,
        KEY_LEFTALT => Key::AltLeft,
        KEY_SPACE => Key::Space,
        KEY_RIGHTALT => Key::AltRight,
        KEY_RIGHTCTRL => Key::ControlRight,
        KEY_LEFT => Key::ArrowLeft,
        KEY_RIGHT => Key::ArrowRight,
        KEY_UP => Key::ArrowUp,
        KEY_DOWN => Key::ArrowDown,
        KEY_INSERT => Key::Insert,
        KEY_DELETE => Key::Delete,
        KEY_HOME => Key::Home,
        KEY_END => Key::End,
        KEY_PAGEUP => Key::PageUp,
        KEY_PAGEDOWN => Key::PageDown,
        KEY_NUMLOCK => Key::NumLock,
        KEY_KP0 => Key::Numpad0,
        KEY_KP1 => Key::Numpad1,
        KEY_KP2 => Key::Numpad2,
        KEY_KP3 => Key::Numpad3,
        KEY_KP4 => Key::Numpad4,
        KEY_KP5 => Key::Numpad5,
        KEY_KP6 => Key::Numpad6,
        KEY_KP7 => Key::Numpad7,
        KEY_KP8 => Key::Numpad8,
        KEY_KP9 => Key::Numpad9,
        KEY_KPSLASH => Key::NumpadDivide,
        KEY_KPASTERISK => Key::NumpadMultiply,
        KEY_KPMINUS => Key::NumpadSubtract,
        KEY_KPPLUS => Key::NumpadAdd,
        KEY_KPENTER => Key::NumpadEnter,
        KEY_KPDOT => Key::NumpadDecimal,
        KEY_PAUSE => Key::Pause,
        KEY_SCROLLLOCK => Key::ScrollLock,
        KEY_PRINT => Key::PrintScreen,
        _ => Key::Space,
    }
}