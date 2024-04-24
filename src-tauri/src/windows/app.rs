use std::{mem, sync::MutexGuard};

use windows::Win32::{
    self,
    Foundation::HWND,
    UI::{
        Input::KeyboardAndMouse::{
            SendInput, INPUT, KEYBDINPUT, KEYBD_EVENT_FLAGS, KEYEVENTF_KEYUP, MAP_VIRTUAL_KEY_TYPE,
            VIRTUAL_KEY,
        },
        WindowsAndMessaging::{GetForegroundWindow, GetMessageExtraInfo, SetForegroundWindow},
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
    update_hwnd(state)
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

pub fn run_macro(state: &mut MutexGuard<OSApp>) {
    unsafe {
        SetForegroundWindow(state.handles.target);
    }

    const CBSIZE: i32 = mem::size_of::<INPUT>() as i32;
    let extra_info = unsafe { GetMessageExtraInfo().0.unsigned_abs() };

    let mut pinputs: &[INPUT] = &[
        INPUT {
            r#type: Win32::UI::Input::KeyboardAndMouse::INPUT_KEYBOARD,
            Anonymous: Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: Win32::UI::Input::KeyboardAndMouse::VK_CONTROL,
                    dwExtraInfo: extra_info,
                    dwFlags: KEYBD_EVENT_FLAGS(0),
                    time: 0,
                    wScan: 1,
                },
            },
        },
        INPUT {
            r#type: Win32::UI::Input::KeyboardAndMouse::INPUT_KEYBOARD,
            Anonymous: Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: Win32::UI::Input::KeyboardAndMouse::VK_R,
                    dwExtraInfo: extra_info,
                    dwFlags: KEYBD_EVENT_FLAGS(0),
                    time: 0,
                    wScan: 1,
                },
            },
        },
        INPUT {
            r#type: Win32::UI::Input::KeyboardAndMouse::INPUT_KEYBOARD,
            Anonymous: Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: Win32::UI::Input::KeyboardAndMouse::VK_R,
                    dwExtraInfo: extra_info,
                    dwFlags: KEYEVENTF_KEYUP,
                    time: 0,
                    wScan: 1,
                },
            },
        },
        INPUT {
            r#type: Win32::UI::Input::KeyboardAndMouse::INPUT_KEYBOARD,
            Anonymous: Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: Win32::UI::Input::KeyboardAndMouse::VK_CONTROL,
                    dwExtraInfo: extra_info,
                    dwFlags: KEYEVENTF_KEYUP,
                    time: 0,
                    wScan: 1,
                },
            },
        },
    ];

    let sent: u32;
    unsafe {
        sent = SendInput(&mut pinputs, CBSIZE);
    }
    println!("Inputs: {}", sent);
}
