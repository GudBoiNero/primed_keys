use priomutex::MutexGuard;

use windows::Win32::{
    Foundation::{GetLastError, HWND, LPARAM, WPARAM},
    System::Threading::{AttachThreadInput, GetCurrentProcess, GetCurrentThreadId},
    UI::{
        Input::KeyboardAndMouse::{GetActiveWindow, SetActiveWindow, VK_CONTROL, VK_R},
        WindowsAndMessaging::{
            BringWindowToTop, GetForegroundWindow, GetWindowThreadProcessId, SendMessageA,
            SendMessageW, SetForegroundWindow, ShowWindow, SHOW_WINDOW_CMD, WM_KEYDOWN, WM_KEYUP,
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
        let mut ret = Self {
            initialized: false,
            handles: WinHandles::default(),
        };

        ret.handles.app = unsafe { GetForegroundWindow() };

        ret
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

/// https://stackoverflow.com/a/59659421/17763366
unsafe fn force_foreground_window(hwnd: HWND) {
    let window_thread_process_id =
        GetWindowThreadProcessId(GetForegroundWindow(), Some(std::ptr::null_mut()));
    let current_thread_id = GetCurrentThreadId();
    const CONST_SW_SHOW: i32 = 5;
    AttachThreadInput(window_thread_process_id, current_thread_id, true);
    println!("{:?}", GetLastError());
    let _ = BringWindowToTop(hwnd);
    ShowWindow(hwnd, SHOW_WINDOW_CMD(CONST_SW_SHOW));
    AttachThreadInput(window_thread_process_id, current_thread_id, false);
    println!("{:?}", GetLastError());
}

pub fn run_macro(state: &mut MutexGuard<OSApp>) {
    unsafe {
        force_foreground_window(state.handles.target);
    }
}
