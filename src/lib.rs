use std::io::{self, Write};

#[derive(Debug)]
pub enum Key {
    Escape,
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
    Grave,
    Digit1, Digit2, Digit3, Digit4, Digit5, Digit6, Digit7, Digit8, Digit9, Digit0,
    Minus, Equal,
    Backspace,
    Tab,
    KeyQ, KeyW, KeyE, KeyR, KeyT, KeyY, KeyU, KeyI, KeyO, KeyP,
    LeftBracket, RightBracket, Backslash,
    CapsLock,
    KeyA, KeyS, KeyD, KeyF, KeyG, KeyH, KeyJ, KeyK, KeyL,
    Semicolon, Quote, Enter,
    ShiftLeft, KeyZ, KeyX, KeyC, KeyV, KeyB, KeyN, KeyM,
    Comma, Period, Slash, ShiftRight,
    ControlLeft, AltLeft, Space, AltRight, ControlRight,
    ArrowLeft, ArrowRight, ArrowUp, ArrowDown,
    Insert, Delete, Home, End, PageUp, PageDown,
    NumLock,
    Numpad0, Numpad1, Numpad2, Numpad3, Numpad4, Numpad5, Numpad6, Numpad7, Numpad8, Numpad9,
    NumpadDivide, NumpadMultiply, NumpadSubtract, NumpadAdd, NumpadEnter, NumpadDecimal,
    PrintScreen, ScrollLock, Pause,
}

pub struct KeyEvent {
    pub key: Key,
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
}

pub fn get_key(prompt: &str) -> io::Result<KeyEvent> {
    eprint!("{}", prompt);
    io::stderr().flush()?;
    platform::wait_key_with_mods()
}

mod platform;