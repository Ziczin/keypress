use crate::{Key, KeyEvent};
use core_foundation::base::TCFType;
use core_foundation::runloop::{CFRunLoop, CFRunLoopRun, CFRunLoopStop, kCFRunLoopDefaultMode};
use core_graphics::event::{CGEvent, CGEventTap, CGEventTapLocation, CGEventTapPlaceholder, CGEventTapOptions, CGEventType};
use core_graphics::event_source::CGEventSourceStateID;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::io;

pub fn wait_key_with_mods() -> io::Result<KeyEvent> {
    let (tx, rx): (Sender<KeyEvent>, Receiver<KeyEvent>) = channel();

    let tap = match CGEventTap::new(
        CGEventTapLocation::HID,
        CGEventTapPlaceholder::Tap,
        CGEventTapOptions::Default,
        vec![CGEventType::KeyDown],
        |_, event_type, event| {
            if event_type == CGEventType::KeyDown {
                let keycode = event.get_integer_value_field(::core_graphics::event::kCGKeyboardEventKeycode) as u16;
                let flags = event.get_flags();
                let shift = flags.contains(::core_graphics::event::CGEventFlags::CGEventFlagMaskShift);
                let ctrl = flags.contains(::core_graphics::event::CGEventFlags::CGEventFlagMaskControl);
                let alt = flags.contains(::core_graphics::event::CGEventFlags::CGEventFlagMaskAlternate);
                let key = map_keycode(keycode);

                let _ = tx.send(KeyEvent { key, shift, ctrl, alt });
                unsafe {
                    CFRunLoopStop(CFRunLoop::get_current().as_concrete_TypeRef());
                }
            }
            None
        },
    ) {
        Ok(tap) => tap,
        Err(_) => return super::crossterm_fallback::wait_key_with_mods(),
    };

    let handle = thread::spawn(move || {
        let run_loop = CFRunLoop::get_current();
        tap.enable();
        run_loop.run();
    });

    let event = rx.recv().map_err(|_| io::Error::new(io::ErrorKind::Other, "no event"))?;
    handle.join().ok();
    Ok(event)
}

fn map_keycode(code: u16) -> Key {
    match code {
        0x00 => Key::KeyA,
        0x01 => Key::KeyS,
        0x02 => Key::KeyD,
        0x03 => Key::KeyF,
        0x04 => Key::KeyH,
        0x05 => Key::KeyG,
        0x06 => Key::KeyZ,
        0x07 => Key::KeyX,
        0x08 => Key::KeyC,
        0x09 => Key::KeyV,
        0x0A => Key::KeyB,
        0x0B => Key::KeyQ,
        0x0C => Key::KeyW,
        0x0D => Key::KeyE,
        0x0E => Key::KeyR,
        0x0F => Key::KeyY,
        0x10 => Key::KeyT,
        0x11 => Key::Digit1,
        0x12 => Key::Digit2,
        0x13 => Key::Digit3,
        0x14 => Key::Digit4,
        0x15 => Key::Digit6,
        0x16 => Key::Digit5,
        0x17 => Key::Equal,
        0x18 => Key::Digit9,
        0x19 => Key::Digit7,
        0x1A => Key::Minus,
        0x1B => Key::Digit8,
        0x1C => Key::Digit0,
        0x1D => Key::RightBracket,
        0x1E => Key::KeyO,
        0x1F => Key::KeyU,
        0x20 => Key::LeftBracket,
        0x21 => Key::KeyI,
        0x22 => Key::KeyP,
        0x23 => Key::Enter,
        0x24 => Key::KeyL,
        0x25 => Key::KeyJ,
        0x26 => Key::Quote,
        0x27 => Key::KeyK,
        0x28 => Key::Semicolon,
        0x29 => Key::Backslash,
        0x2A => Key::Comma,
        0x2B => Key::Slash,
        0x2C => Key::KeyN,
        0x2D => Key::KeyM,
        0x2E => Key::Period,
        0x2F => Key::Tab,
        0x30 => Key::Space,
        0x31 => Key::Grave,
        0x32 => Key::Backspace,
        0x33 => Key::Escape,
        0x34 => Key::ArrowRight,
        0x35 => Key::ArrowLeft,
        0x36 => Key::ArrowDown,
        0x37 => Key::ArrowUp,
        0x38 => Key::ShiftLeft,
        0x39 => Key::CapsLock,
        0x3A => Key::AltLeft,
        0x3B => Key::ControlLeft,
        0x3C => Key::ShiftRight,
        0x3D => Key::AltRight,
        0x3E => Key::ControlRight,
        0x3F => Key::F17,
        0x40 => Key::F18,
        0x41 => Key::F19,
        0x42 => Key::F20,
        0x43 => Key::F5,
        0x44 => Key::F6,
        0x45 => Key::F7,
        0x46 => Key::F3,
        0x47 => Key::F8,
        0x48 => Key::F9,
        0x49 => Key::F10,
        0x4A => Key::F11,
        0x4B => Key::F12,
        0x4C => Key::F13,
        0x4D => Key::F14,
        0x4E => Key::F15,
        0x4F => Key::F16,
        0x50 => Key::F17,
        0x51 => Key::F18,
        0x52 => Key::F19,
        0x53 => Key::F20,
        0x54 => Key::F21,
        0x55 => Key::F22,
        0x56 => Key::F23,
        0x57 => Key::F24,
        _ => Key::Space,
    }
}