use std::sync::MutexGuard;

use windows::Win32::Foundation::HWND;

use crate::platforms::app::App;

#[derive(Default)]
pub struct WinHandles {
    pub current: HWND,
    pub previous: HWND,
    pub target: HWND,
}

pub struct WinApp {
    pub initialized: bool,
    pub handles: WinHandles,
}
impl App for WinApp {
    fn new() -> Self {
        Self {
            initialized: false,
            handles: WinHandles::default(),
        }
    }

    fn initialized(&self) -> bool {
        self.initialized
    }
}

pub fn impl_win_update(state: &mut MutexGuard<WinApp>) {}
