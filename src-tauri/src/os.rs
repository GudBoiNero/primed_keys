// OS Independent Interface
use std::sync::MutexGuard;

pub trait App {
    fn new() -> Self;
    fn initialized(&self) -> bool;
}

#[cfg(target_os = "windows")]
pub type OSApp = crate::windows::app::WinApp;
#[cfg(target_os = "linux")]
pub type OSApp = crate::linux::app::LinuxApp;
#[cfg(target_os = "macos")]
pub type OSApp = crate::macos::app::MacApp;

pub fn update(state: &mut MutexGuard<OSApp>) {
    #[cfg(target_os = "windows")]
    super::windows::app::impl_win_update(state);
    #[cfg(target_os = "linux")]
    super::linux::app::impl_linux_update(state);
    #[cfg(target_os = "macos")]
    super::macos::app::impl_macos_update(state);
}
