use std::{
    mem::size_of,
    ptr::{null, null_mut},
};

use priomutex::MutexGuard;

use windows::Win32::{
    Foundation::{BOOL, HWND, LPARAM, POINT, WPARAM},
    System::Threading::{AttachThreadInput, GetCurrentThreadId},
    UI::{
        Input::KeyboardAndMouse::{SendInput, INPUT},
        WindowsAndMessaging::{
            BringWindowToTop, DispatchMessageW, GetForegroundWindow, GetMessageExtraInfo,
            GetMessageW, GetWindowThreadProcessId, PeekMessageW, ShowWindow, TranslateMessage, MSG,
            PM_REMOVE, SHOW_WINDOW_CMD,
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
    pub msg: MSG,
}
impl App for OSApp {
    fn new() -> Self {
        Self {
            initialized: false,
            handles: WinHandles::default(),
            msg: MSG {
                hwnd: HWND(0),
                message: 0,
                wParam: WPARAM(0),
                lParam: LPARAM(0),
                time: 0,
                pt: POINT {
                    ..Default::default()
                },
            },
        }
    }

    fn initialized(&self) -> bool {
        self.initialized
    }
}

pub fn update(state: &mut MutexGuard<OSApp>) {
    assert!(state.handles.app != state.handles.target);

    message_loop(state);
    update_hwnd(state);
}

fn message_loop(state: &mut MutexGuard<OSApp>) {
    unsafe {
        let pm = GetMessageW(&mut state.msg, state.handles.app, 0, 0);
        if pm.as_bool() {
            println!("Message Loop");
            TranslateMessage(&state.msg);
            DispatchMessageW(&state.msg);
        }
    }
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
        println!("Changed Target: {:?} to {:?}", state.handles.target, prev);
    }
}

/// Source: https://stackoverflow.com/a/59659421/17763366
unsafe fn force_foreground_window(hwnd: HWND) {
    let current_thread_id = GetCurrentThreadId();
    let window_thread_process_id = GetWindowThreadProcessId(hwnd, Some(null_mut()));
    const CONST_SW_SHOW: i32 = 5;
    if AttachThreadInput(current_thread_id, window_thread_process_id, BOOL(1)).as_bool() {
        let _ = BringWindowToTop(hwnd);
        ShowWindow(hwnd, SHOW_WINDOW_CMD(CONST_SW_SHOW));
        AttachThreadInput(current_thread_id, window_thread_process_id, BOOL(0));
    }
}

pub fn run_macro(state: &mut MutexGuard<OSApp>) {
    unsafe {
        force_foreground_window(state.handles.target);

        const CBSIZE: i32 = size_of::<INPUT>() as i32;
        let mut pinputs: &[INPUT] = &get_inputs(state);
        SendInput(&mut pinputs, CBSIZE);
    }
}

pub fn get_inputs(state: &mut MutexGuard<OSApp>) -> Box<[INPUT]> {
    let _extra_info = unsafe { GetMessageExtraInfo().0.unsigned_abs() };

    Box::new([])
}
