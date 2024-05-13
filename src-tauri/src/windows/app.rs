use std::{mem::size_of, ptr::null_mut};

use priomutex::MutexGuard;

use windows::{
    core::PCSTR,
    Win32::{
        Foundation::{BOOL, HMODULE, HWND, LPARAM, LRESULT, POINT, WPARAM},
        System::{
            LibraryLoader::{
                GetModuleHandleExA, GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS,
                GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
            },
            Threading::{AttachThreadInput, GetCurrentThreadId},
        },
        UI::{
            Input::KeyboardAndMouse::{SendInput, INPUT},
            WindowsAndMessaging::{
                BringWindowToTop, DispatchMessageW, GetForegroundWindow, GetMessageExtraInfo,
                GetWindowThreadProcessId, PeekMessageW, SetWindowsHookExA, ShowWindow,
                TranslateMessage, UnhookWindowsHookEx, HHOOK, MSG, PM_REMOVE, SHOW_WINDOW_CMD,
                WH_KEYBOARD_LL,
            },
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
    pub hook_id: HHOOK,
    pub msg: MSG,
}
impl App for OSApp {
    fn new() -> Self {
        let hook_id = unsafe {
            let hmodule: &mut HMODULE = &mut HMODULE(0);
            let _ = GetModuleHandleExA(
                GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS
                    | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
                PCSTR(null_mut()),
                hmodule,
            );
            SetWindowsHookExA(WH_KEYBOARD_LL, Some(wndproc), *hmodule, 0).unwrap()
        };

        Self {
            initialized: false,
            hook_id,
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

impl Drop for OSApp {
    fn drop(&mut self) {
        unsafe {
            let _ = UnhookWindowsHookEx(self.hook_id);
        };
    }
}

pub fn update(state: &mut MutexGuard<OSApp>) {
    /// Even at initialization the app handle should never be the same as the target handle
    /// If it somehow is, and continues running, the program will not work correctly.
    assert!(
        state.handles.app != state.handles.target,
        "app: {:?}, target: {:?}",
        state.handles.app,
        state.handles.target
    );

    message_loop(state);
    update_hwnd(state);
}

fn message_loop(state: &mut MutexGuard<OSApp>) {
    unsafe {
        let pm = PeekMessageW(&mut state.msg, state.handles.app, 0, 0, PM_REMOVE);
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

extern "system" fn wndproc(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    println!("wndproc: ({:?}, {:?}, {:?})", code, wparam, lparam);
    LRESULT(0)
}
