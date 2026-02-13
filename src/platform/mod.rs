#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub use windows::wait_key_with_mods;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::wait_key_with_mods;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::wait_key_with_mods;

pub(crate) mod crossterm_fallback;

#[cfg(not(any(windows, target_os = "linux", target_os = "macos")))]
pub use crossterm_fallback::wait_key_with_mods;