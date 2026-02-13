use keypress::{get_key, Key};

fn main() -> std::io::Result<()> {
    loop {
        let event = get_key("Press any key (ESC to exit): ")?;
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