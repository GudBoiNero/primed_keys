use std::sync::MutexGuard;

use crate::platforms::app::App;

pub struct WinApp {
    pub initialized: bool,
}
impl App for WinApp {
    fn new() -> Self {
        Self { initialized: false }
    }

    fn initialized(&self) -> bool {
        self.initialized
    }
}

pub fn impl_win_update(state: &mut MutexGuard<WinApp>) {}
