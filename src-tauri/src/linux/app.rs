use std::sync::MutexGuard;

use crate::app::App;

pub struct OSApp {
    pub initialized: bool,
}
impl App for OSApp {
    fn new() -> Self {
        Self { initialized: false }
    }

    fn initialized(&self) -> bool {
        self.initialized
    }
}

pub fn update(state: &mut MutexGuard<OSApp>) {}
