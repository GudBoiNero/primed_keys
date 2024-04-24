use std::{mem, sync::MutexGuard};

use windows::Win32::{
    self,
    Foundation::{GetLastError, HWND, LPARAM, WPARAM},
    System::Threading::AttachThreadInput,
    UI::{
        Input::KeyboardAndMouse::{
            SendInput, SetActiveWindow, SetFocus, INPUT, KEYBDINPUT, KEYBD_EVENT_FLAGS,
            KEYEVENTF_KEYUP, MAP_VIRTUAL_KEY_TYPE, VIRTUAL_KEY, VK_LWIN,
        },
        WindowsAndMessaging::{
            GetForegroundWindow, GetMessageExtraInfo, GetWindowThreadProcessId, PostMessageA,
            PostMessageW, SendMessageA, SendMessageW, SetForegroundWindow, WM_KEYDOWN,
        },
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
    update_msg_threads(state);
}

fn update_hwnd(state: &mut MutexGuard<OSApp>) {
    state.handles.prev = state.handles.curr;
    state.handles.curr = unsafe { GetForegroundWindow() };

    if state.handles.curr != state.handles.app
        && state.handles.curr != state.handles.prev
        && state.handles.curr.0 != 0
    {
        state.handles.target = state.handles.curr;
    }
}

fn update_msg_threads(state: &mut MutexGuard<OSApp>) {}

pub fn run_macro(state: &mut MutexGuard<OSApp>) {
    unsafe {
        // This doesn't work. I need to do this so I can properly send inputs.
        SetForegroundWindow(state.handles.target);
        // This should send the Left Window key press to the target handle window.
        // For some reason there's no error or input showing up.
        SendMessageW(
            state.handles.target,
            WM_KEYDOWN,
            WPARAM(VK_LWIN.0.into()),
            LPARAM(0),
        );
        println!("Result: {:?}", GetLastError());
    }
}
