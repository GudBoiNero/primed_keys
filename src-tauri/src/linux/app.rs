use std::sync::MutexGuard;

use crate::platforms::app::App;

pub struct LinuxApp {
    pub initialized: bool,
}
impl App for LinuxApp {
    fn new() -> Self {
        Self { initialized: false }
    }

    fn initialized(&self) -> bool {
        self.initialized
    }
}

pub fn impl_linux_update(state: &mut MutexGuard<LinuxApp>) {}
