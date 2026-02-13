use keypress::{wait_key_with_mods, Key};

fn main() -> std::io::Result<()> {
    loop {
        let event = wait_key_with_mods("Press any key (ESC to exit): ")?;
        println!(
            "Key: {:?}, Shift: {}, Ctrl: {}, Alt: {}",
            event.key, event.shift, event.ctrl, event.alt
        );
        if matches!(event.key, Key::Escape) {
            break;
        }
    }
    Ok(())
}