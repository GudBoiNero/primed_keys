use std::sync::MutexGuard;

use super::windows::app::{impl_win_update, WinApp};

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

pub fn update(state: &mut MutexGuard<PlatApp>) {
    #[cfg(target_os = "windows")]
    impl_win_update(state);
    #[cfg(target_os = "linux")]
    impl_linux_update(state);
    #[cfg(target_os = "macos")]
    impl_macos_update(state);
}
