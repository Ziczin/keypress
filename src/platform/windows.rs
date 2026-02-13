use crate::{Key, KeyEvent};
use std::io;
use winapi::shared::minwindef::{DWORD, UINT};
use winapi::um::consoleapi::{ReadConsoleInputW, SetConsoleMode, GetConsoleMode};
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::um::processenv::GetStdHandle;
use winapi::um::winbase::STD_INPUT_HANDLE;
use winapi::um::wincon::{INPUT_RECORD, KEY_EVENT,
                         LEFT_CTRL_PRESSED, RIGHT_CTRL_PRESSED,
                         LEFT_ALT_PRESSED, RIGHT_ALT_PRESSED,
                         SHIFT_PRESSED, ENHANCED_KEY};
use winapi::um::winuser::*;
use winapi::um::winnt::HANDLE;

struct ConsoleModeGuard(HANDLE, DWORD);

impl ConsoleModeGuard {
    fn new(h: HANDLE) -> io::Result<Self> {
        let mut original = 0;
        if unsafe { GetConsoleMode(h, &mut original) } == 0 {
            return Err(io::Error::last_os_error());
        }
        if unsafe { SetConsoleMode(h, 0) } == 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(ConsoleModeGuard(h, original))
    }
}

impl Drop for ConsoleModeGuard {
    fn drop(&mut self) {
        unsafe { SetConsoleMode(self.0, self.1); }
    }
}

pub fn wait_key_with_mods() -> io::Result<KeyEvent> {
    unsafe {
        let h_stdin = GetStdHandle(STD_INPUT_HANDLE);
        if h_stdin == INVALID_HANDLE_VALUE {
            return super::crossterm_fallback::wait_key_with_mods();
        }

        let _guard = match ConsoleModeGuard::new(h_stdin) {
            Ok(g) => g,
            Err(_) => return super::crossterm_fallback::wait_key_with_mods(),
        };

        let mut record: INPUT_RECORD = std::mem::zeroed();
        let mut events_read: DWORD = 0;
        
        let mut shift_pressed = false;
        let mut ctrl_pressed = false;
        let mut alt_pressed = false;

        loop {
            if ReadConsoleInputW(h_stdin, &mut record, 1, &mut events_read) == 0 {
                continue;
            }
            if events_read == 0 || record.EventType != KEY_EVENT {
                continue;
            }

            let ke = *record.Event.KeyEvent();
            let vk = ke.wVirtualKeyCode;
            let scan = ke.wVirtualScanCode;
            let state = ke.dwControlKeyState;
            let is_key_down = ke.bKeyDown != 0;

            shift_pressed = (state & SHIFT_PRESSED) != 0;
            ctrl_pressed = (state & (LEFT_CTRL_PRESSED | RIGHT_CTRL_PRESSED)) != 0;
            alt_pressed = (state & (LEFT_ALT_PRESSED | RIGHT_ALT_PRESSED)) != 0;

            if vk as i32 == VK_SHIFT || vk as i32 == VK_CONTROL || vk as i32 == VK_MENU {
                continue;
            }

            if !is_key_down {
                continue;
            }

            let key = map_scan(scan as UINT, vk as UINT, state);

            return Ok(KeyEvent { 
                key, 
                shift: shift_pressed, 
                ctrl: ctrl_pressed, 
                alt: alt_pressed 
            });
        }
    }
}

fn map_scan(scan: UINT, vk: UINT, state: DWORD) -> Key {
    match scan {
        0x01 => Key::Escape,
        0x3B => Key::F1, 0x3C => Key::F2, 0x3D => Key::F3, 0x3E => Key::F4,
        0x3F => Key::F5, 0x40 => Key::F6, 0x41 => Key::F7, 0x42 => Key::F8,
        0x43 => Key::F9, 0x44 => Key::F10, 0x57 => Key::F11, 0x58 => Key::F12,
        0x29 => Key::Grave,
        0x02 => Key::Digit1, 0x03 => Key::Digit2, 0x04 => Key::Digit3, 0x05 => Key::Digit4,
        0x06 => Key::Digit5, 0x07 => Key::Digit6, 0x08 => Key::Digit7, 0x09 => Key::Digit8,
        0x0A => Key::Digit9, 0x0B => Key::Digit0,
        0x0C => Key::Minus, 0x0D => Key::Equal,
        0x0E => Key::Backspace,
        0x0F => Key::Tab,
        0x10 => Key::KeyQ, 0x11 => Key::KeyW, 0x12 => Key::KeyE, 0x13 => Key::KeyR,
        0x14 => Key::KeyT, 0x15 => Key::KeyY, 0x16 => Key::KeyU, 0x17 => Key::KeyI,
        0x18 => Key::KeyO, 0x19 => Key::KeyP,
        0x1A => Key::LeftBracket, 0x1B => Key::RightBracket, 0x2B => Key::Backslash,
        0x3A => Key::CapsLock,
        0x1E => Key::KeyA, 0x1F => Key::KeyS, 0x20 => Key::KeyD, 0x21 => Key::KeyF,
        0x22 => Key::KeyG, 0x23 => Key::KeyH, 0x24 => Key::KeyJ, 0x25 => Key::KeyK,
        0x26 => Key::KeyL,
        0x27 => Key::Semicolon, 0x28 => Key::Quote, 0x1C => Key::Enter,
        0x2A => Key::ShiftLeft,
        0x2C => Key::KeyZ, 0x2D => Key::KeyX, 0x2E => Key::KeyC, 0x2F => Key::KeyV,
        0x30 => Key::KeyB, 0x31 => Key::KeyN, 0x32 => Key::KeyM,
        0x33 => Key::Comma, 0x34 => Key::Period, 0x35 => Key::Slash, 0x36 => Key::ShiftRight,
        0x1D => {
            if (state & ENHANCED_KEY) != 0 { Key::ControlRight } else { Key::ControlLeft }
        },
        0x38 => {
            if (state & ENHANCED_KEY) != 0 { Key::AltRight } else { Key::AltLeft }
        },
        0x39 => Key::Space,
        0x47 => Key::Home, 0x48 => Key::ArrowUp, 0x49 => Key::PageUp,
        0x4B => Key::ArrowLeft, 0x4D => Key::ArrowRight,
        0x4F => Key::End, 0x50 => Key::ArrowDown, 0x51 => Key::PageDown,
        0x52 => Key::Insert, 0x53 => Key::Delete,
        0x45 => Key::NumLock,
        0x37 => Key::NumpadMultiply, 0x4A => Key::NumpadSubtract,
        0x4E => Key::NumpadAdd, 0x1C => {
            if (state & ENHANCED_KEY) != 0 { Key::NumpadEnter } else { Key::Enter }
        },
        0x53 => Key::NumpadDecimal,
        0x35 => {
            if (state & ENHANCED_KEY) != 0 { Key::NumpadDivide } else { Key::Slash }
        },
        0x47 => Key::Home, 0x48 => Key::ArrowUp, 0x49 => Key::PageUp,
        0x4B => Key::ArrowLeft, 0x4D => Key::ArrowRight,
        0x4F => Key::End, 0x50 => Key::ArrowDown, 0x51 => Key::PageDown,
        0x52 => Key::Insert, 0x53 => Key::Delete,
        _ => {
            match vk as i32 {
                VK_NUMPAD0 => Key::Numpad0, VK_NUMPAD1 => Key::Numpad1,
                VK_NUMPAD2 => Key::Numpad2, VK_NUMPAD3 => Key::Numpad3,
                VK_NUMPAD4 => Key::Numpad4, VK_NUMPAD5 => Key::Numpad5,
                VK_NUMPAD6 => Key::Numpad6, VK_NUMPAD7 => Key::Numpad7,
                VK_NUMPAD8 => Key::Numpad8, VK_NUMPAD9 => Key::Numpad9,
                _ => Key::Space,
            }
        },
    }
}