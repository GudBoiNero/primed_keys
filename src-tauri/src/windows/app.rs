use std::ptr::null_mut;

use priomutex::MutexGuard;

use windows::Win32::{
    Foundation::{GetLastError, BOOL, HWND},
    System::Threading::{AttachThreadInput, GetCurrentThreadId},
    UI::WindowsAndMessaging::{
        BringWindowToTop, GetForegroundWindow, GetWindowThreadProcessId, ShowWindow,
        SHOW_WINDOW_CMD,
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
        println!("Changed Target: {} to {}", state.handles.target.0, prev.0);
    }
}

/// `AttachThreadInput` seems to have some issues in Rust. It always returns `WIN32_ERROR(87)` in this case. \
/// Source: https://stackoverflow.com/a/59659421/17763366
unsafe fn force_foreground_window(hwnd: HWND) {
    let current_thread_id = GetCurrentThreadId();
    let window_thread_process_id =
        GetWindowThreadProcessId(GetForegroundWindow(), Some(null_mut()));
    const CONST_SW_SHOW: i32 = 5;
    if AttachThreadInput(current_thread_id, window_thread_process_id, BOOL(1)).as_bool() {
        let _ = BringWindowToTop(hwnd);
        ShowWindow(hwnd, SHOW_WINDOW_CMD(CONST_SW_SHOW));
        AttachThreadInput(current_thread_id, window_thread_process_id, BOOL(0));
    }
    println!("Error: {:?}", GetLastError());
}

pub fn run_macro(state: &mut MutexGuard<OSApp>) {
    unsafe {
        force_foreground_window(state.handles.target);
    }
}
