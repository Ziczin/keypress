# keypress

Cross-platform library for waiting for a physical key press with modifier detection.

## Features

- Returns physical key and modifier state
- Support for Windows, Linux, macOS
- Automatic fallback to crossterm

## Usage

```rust
use keypress::{get_key, Key};

fn main() -> std::io::Result<()> {
    let event = get_key("Press a key: ")?;
    println!("Key: {:?}", event.key);
    Ok(())
}
```

## Note

Tested on Windows only. Linux and macOS implementations are provided but may require additional testing and configuration.