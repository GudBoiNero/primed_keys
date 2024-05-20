use std::{
    f64::RADIX,
    ffi::c_short,
    mem::{self, size_of},
    ptr::null_mut,
};

use priomutex::MutexGuard;

use windows::Win32::{
    Foundation::{BOOL, HWND},
    System::Threading::{AttachThreadInput, GetCurrentThreadId},
    UI::{
        Input::KeyboardAndMouse::{
            GetAsyncKeyState, MapVirtualKeyA, SendInput, INPUT, INPUT_0, INPUT_TYPE, KEYBDINPUT,
            KEYBD_EVENT_FLAGS, KEYEVENTF_EXTENDEDKEY, MAPVK_VK_TO_CHAR, MAPVK_VK_TO_VSC,
            MAPVK_VK_TO_VSC_EX, VIRTUAL_KEY, VK_ADD, VK_BACK, VK_CAPITAL, VK_CONTROL, VK_DECIMAL,
            VK_DELETE, VK_DIVIDE, VK_DOWN, VK_END, VK_ESCAPE, VK_F1, VK_F10, VK_F11, VK_F12, VK_F2,
            VK_F3, VK_F4, VK_F5, VK_F6, VK_F7, VK_F8, VK_F9, VK_HOME, VK_INSERT, VK_LBUTTON,
            VK_LEFT, VK_MBUTTON, VK_MENU, VK_MULTIPLY, VK_NEXT, VK_NUMPAD0, VK_NUMPAD1, VK_NUMPAD2,
            VK_NUMPAD3, VK_NUMPAD4, VK_NUMPAD5, VK_NUMPAD6, VK_NUMPAD7, VK_NUMPAD8, VK_NUMPAD9,
            VK_PAUSE, VK_PRIOR, VK_RBUTTON, VK_RETURN, VK_RIGHT, VK_SEPARATOR, VK_SHIFT, VK_SPACE,
            VK_SUBTRACT, VK_TAB, VK_UP, VK_XBUTTON1, VK_XBUTTON2,
        },
        WindowsAndMessaging::{
            BringWindowToTop, GetForegroundWindow, GetMessageExtraInfo, GetWindowThreadProcessId,
            ShowWindow, SHOW_WINDOW_CMD,
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
    /// Even at initialization the app handle should never be the same as the target handle
    /// If it somehow is, and continues running, the program will not work correctly.
    assert!(
        state.handles.app != state.handles.target,
        "app: {:?}, target: {:?}",
        state.handles.app,
        state.handles.target
    );

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

pub fn run_macro(state: &mut MutexGuard<OSApp>, name: String) {
    unsafe {
        force_foreground_window(state.handles.target);

        const CBSIZE: i32 = size_of::<INPUT>() as i32;
        let inputs = &get_macro_inputs(state, name);
        let mut pinputs: &[INPUT] = inputs;
        SendInput(&mut pinputs, CBSIZE);
    }
}

/// Takes a `Vec<u32>` and initializes it as a vector of `INPUT`s.
fn make_inputs(state: &mut MutexGuard<OSApp>, keys: Vec<u32>) -> Vec<INPUT> {
    let _extra_info = unsafe { GetMessageExtraInfo().0.unsigned_abs() };
    let mut inputs: Vec<INPUT> = vec![];

    keys.iter().for_each(|vk| {
        let input = INPUT {
            r#type: INPUT_TYPE { 0: 1 },
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VIRTUAL_KEY(*vk as u16),
                    wScan: 1,
                    dwFlags: KEYEVENTF_EXTENDEDKEY,
                    time: 0,
                    dwExtraInfo: _extra_info,
                },
            },
        };

        inputs.push(input)
    });

    inputs
}

/// Read from json config file and converts to `Vec<INPUT>`
fn get_macro_inputs(state: &mut MutexGuard<OSApp>, name: String) -> Vec<INPUT> {
    let _extra_info = unsafe { GetMessageExtraInfo().0.unsigned_abs() };
    let mut inputs: Vec<INPUT> = vec![];

    inputs
}

/// Gets all `VIRTUAL_KEY`s pressed by the user currently.
unsafe fn get_pressed_keys() -> Vec<u32> {
    let mut keys = vec![];

    for vk in 0x01..0xFE {
        let key = GetAsyncKeyState(vk);
        if (key & (0x8000u16 as i16)) != 0 {
            keys.push(vk as u32)
        }
    }

    keys
}
