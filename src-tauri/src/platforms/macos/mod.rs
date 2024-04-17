use std::sync::MutexGuard;

use crate::platforms::app::App;

pub struct MacApp {
    pub initialized: bool,
}
impl App for MacApp {
    fn new() -> Self {
        Self { initialized: false }
    }

    fn initialized(&self) -> bool {
        self.initialized
    }
}

pub fn impl_macos_update(state: &mut MutexGuard<MacApp>) {}
