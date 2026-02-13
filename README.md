# keypress

Cross-platform library for waiting for a physical key press with modifier detection.

## Features

- Returns physical key and modifier state
- Support for Windows, Linux, macOS
- Automatic fallback to crossterm

## Usage

```rust
use keypress::{wait_key_with_mods, Key};

fn main() -> std::io::Result<()> {
    let event = wait_key_with_mods("Press a key: ")?;
    println!("Key: {:?}", event.key);
    Ok(())
}
```

## Note

Tested on Windows only. Linux and macOS implementations are provided but may require additional testing and configuration.