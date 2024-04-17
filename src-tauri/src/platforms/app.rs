use std::sync::MutexGuard;

use super::windows::app::WinApp;

pub trait App {
    fn new() -> Self;
    fn initialized(&self) -> bool;
}

#[cfg(target_os = "windows")]
pub type PlatApp = WinApp;
#[cfg(target_os = "linux")]
pub type PlatApp = LinuxApp;
#[cfg(target_os = "macos")]
pub type PlatApp = MacApp;

pub fn runtime(state: &mut MutexGuard<PlatApp>) {}
