use std::sync::MutexGuard;

pub trait App {
    fn new() -> Self;
    fn initialized(&self) -> bool;
}

#[cfg(target_os = "windows")]
pub type PlatApp = crate::windows::app::WinApp;
#[cfg(target_os = "linux")]
pub type PlatApp = super::linux::LinuxApp;
#[cfg(target_os = "macos")]
pub type PlatApp = super::macos::MacApp;

pub fn update(state: &mut MutexGuard<PlatApp>) {
    #[cfg(target_os = "windows")]
    super::windows::app::impl_win_update(state);
    #[cfg(target_os = "linux")]
    super::linux::app::impl_linux_update(state);
    #[cfg(target_os = "macos")]
    super::macos::app::impl_macos_update(state);
}
