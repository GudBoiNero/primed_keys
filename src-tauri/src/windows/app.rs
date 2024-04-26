use std::sync::MutexGuard;

use windows::Win32::{
    Foundation::{GetLastError, HWND, LPARAM, WPARAM},
    UI::{
        Input::KeyboardAndMouse::{VK_CONTROL, VK_R},
        WindowsAndMessaging::{GetForegroundWindow, SendMessageA, WM_KEYDOWN, WM_KEYUP},
    },
};

use crate::app::App;

#[derive(Default)]
pub struct WinHandles {
    pub curr: HWND,
    pub prev: HWND,
    pub target: HWND,
    pub app: HWND,
}

pub struct OSApp {
    pub initialized: bool,
    pub handles: WinHandles,
}
impl App for OSApp {
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

pub fn update(state: &mut MutexGuard<OSApp>) {
    update_hwnd(state);
}

fn update_hwnd(state: &mut MutexGuard<OSApp>) {
    state.handles.prev = state.handles.curr;
    state.handles.curr = unsafe { GetForegroundWindow() };

    if state.handles.curr != state.handles.app
        && state.handles.curr != state.handles.prev
        && state.handles.curr.0 != 0
    {
        let prev = state.handles.target.clone();
        state.handles.target = state.handles.curr;
        println!("Changed HWND: {} to {}", state.handles.target.0, prev.0);
    }
}

/// TODO: this breaks the main loop.
pub fn run_macro(state: &mut MutexGuard<OSApp>) {
    unsafe {
        // This should send Ctrl+R to the target handle window.
        SendMessageA(
            state.handles.target,
            WM_KEYDOWN,
            WPARAM(VK_CONTROL.0.into()),
            LPARAM(0),
        );
        println!("SendMessageA VK_CONTROL WM_KEYDOWN: {:?}", GetLastError());
        SendMessageA(
            state.handles.target,
            WM_KEYDOWN,
            WPARAM(VK_R.0.into()),
            LPARAM(0),
        );
        println!("SendMessageA VK_R WM_KEYDOWN: {:?}", GetLastError());
        SendMessageA(
            state.handles.target,
            WM_KEYUP,
            WPARAM(VK_R.0.into()),
            LPARAM(0),
        );
        println!("SendMessageA VK_R WM_KEYUP: {:?}", GetLastError());
        SendMessageA(
            state.handles.target,
            WM_KEYUP,
            WPARAM(VK_CONTROL.0.into()),
            LPARAM(0),
        );
        println!("SendMessageA VK_CONTROL WM_KEYUP: {:?}", GetLastError());
    }
}
