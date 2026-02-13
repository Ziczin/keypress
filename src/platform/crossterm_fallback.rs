use crate::{Key, KeyEvent};
use crossterm::event::{self, Event, KeyCode, KeyEvent as CrosstermKeyEvent, KeyModifiers};
use std::io;

pub fn wait_key_with_mods() -> io::Result<KeyEvent> {
    loop {
        match event::read()? {
            Event::Key(CrosstermKeyEvent {
                code,
                modifiers,
                kind: event::KeyEventKind::Press,
                ..
            }) => {
                let key = map_code(code);
                let shift = modifiers.contains(KeyModifiers::SHIFT);
                let ctrl = modifiers.contains(KeyModifiers::CONTROL);
                let alt = modifiers.contains(KeyModifiers::ALT);
                return Ok(KeyEvent { key, shift, ctrl, alt });
            }
            _ => continue,
        }
    }
}

fn map_code(code: KeyCode) -> Key {
    match code {
        KeyCode::Esc => Key::Escape,
        KeyCode::F(n) => match n {
            1 => Key::F1,
            2 => Key::F2,
            3 => Key::F3,
            4 => Key::F4,
            5 => Key::F5,
            6 => Key::F6,
            7 => Key::F7,
            8 => Key::F8,
            9 => Key::F9,
            10 => Key::F10,
            11 => Key::F11,
            12 => Key::F12,
            _ => Key::F12,
        },
        KeyCode::Backspace => Key::Backspace,
        KeyCode::Enter => Key::Enter,
        KeyCode::Left => Key::ArrowLeft,
        KeyCode::Right => Key::ArrowRight,
        KeyCode::Up => Key::ArrowUp,
        KeyCode::Down => Key::ArrowDown,
        KeyCode::Home => Key::Home,
        KeyCode::End => Key::End,
        KeyCode::PageUp => Key::PageUp,
        KeyCode::PageDown => Key::PageDown,
        KeyCode::Tab => Key::Tab,
        KeyCode::Delete => Key::Delete,
        KeyCode::Insert => Key::Insert,
        KeyCode::CapsLock => Key::CapsLock,
        KeyCode::ScrollLock => Key::ScrollLock,
        KeyCode::NumLock => Key::NumLock,
        KeyCode::PrintScreen => Key::PrintScreen,
        KeyCode::Pause => Key::Pause,
        KeyCode::Menu => Key::AltRight,
        KeyCode::Char(' ') => Key::Space,
        KeyCode::Char(c) => {
            if c.is_ascii_digit() {
                match c {
                    '1' => Key::Digit1,
                    '2' => Key::Digit2,
                    '3' => Key::Digit3,
                    '4' => Key::Digit4,
                    '5' => Key::Digit5,
                    '6' => Key::Digit6,
                    '7' => Key::Digit7,
                    '8' => Key::Digit8,
                    '9' => Key::Digit9,
                    '0' => Key::Digit0,
                    _ => unreachable!(),
                }
            } else if c.is_ascii_lowercase() {
                match c {
                    'a' => Key::KeyA,
                    'b' => Key::KeyB,
                    'c' => Key::KeyC,
                    'd' => Key::KeyD,
                    'e' => Key::KeyE,
                    'f' => Key::KeyF,
                    'g' => Key::KeyG,
                    'h' => Key::KeyH,
                    'i' => Key::KeyI,
                    'j' => Key::KeyJ,
                    'k' => Key::KeyK,
                    'l' => Key::KeyL,
                    'm' => Key::KeyM,
                    'n' => Key::KeyN,
                    'o' => Key::KeyO,
                    'p' => Key::KeyP,
                    'q' => Key::KeyQ,
                    'r' => Key::KeyR,
                    's' => Key::KeyS,
                    't' => Key::KeyT,
                    'u' => Key::KeyU,
                    'v' => Key::KeyV,
                    'w' => Key::KeyW,
                    'x' => Key::KeyX,
                    'y' => Key::KeyY,
                    'z' => Key::KeyZ,
                    _ => Key::KeyZ,
                }
            } else {
                match c {
                    '`' | '~' => Key::Grave,
                    '-' | '_' => Key::Minus,
                    '=' | '+' => Key::Equal,
                    '[' | '{' => Key::LeftBracket,
                    ']' | '}' => Key::RightBracket,
                    '\\' | '|' => Key::Backslash,
                    ';' | ':' => Key::Semicolon,
                    '\'' | '"' => Key::Quote,
                    ',' | '<' => Key::Comma,
                    '.' | '>' => Key::Period,
                    '/' | '?' => Key::Slash,
                    _ => Key::Space,
                }
            }
        }
        _ => Key::Space,
    }
}