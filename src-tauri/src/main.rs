#![allow(unused_doc_comments)]
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(target_os = "linux")]
pub(crate) mod linux;
#[cfg(target_os = "macos")]
pub(crate) mod macos;
#[cfg(windows)]
pub(crate) mod windows;

use app::ThreadPriority;
#[cfg(target_os = "linux")]
use linux as os;
#[cfg(target_os = "macos")]
use macos as os;
#[cfg(windows)]
use windows as os;

mod app;

use crate::app::App;
use os::app::OSApp;
use priomutex::Mutex;
use std::{sync::Arc, thread};
use tauri::{Runtime, State};

pub struct AppState(pub Arc<Mutex<OSApp>>);
impl AppState {
    pub fn init() -> Self {
        AppState(Arc::new(Mutex::new(OSApp::new())))
    }
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn run_macro(_name: String, _state: State<AppState>) -> Result<(), ()> {
    // TODO: Turn `_state` to `state` into macro.
    let lock = _state.0.lock(ThreadPriority::Command.into());
    let lock = match lock {
        Ok(x) => x,
        Err(_) => return Err(()),
    };
    let mut state = lock;

    os::app::run_macro(&mut state);

    Ok(())
}

#[tauri::command]
// https://github.com/tauri-apps/tauri/discussions/4775
fn init<R: Runtime>(_window: tauri::Window<R>, state: State<'_, AppState>) -> Result<(), String> {
    let mut init_lock = state.0.lock(ThreadPriority::Main.into()).unwrap();
    if init_lock.initialized {
        println!("App Already Initialized! (Did the page refresh?)");
        return Err("App already initialized.".to_owned());
    } else {
        init_lock.initialized = true;
        if cfg!(windows) {
            init_lock.handles.app = ::windows::Win32::Foundation::HWND(_window.hwnd().unwrap().0);
        }
        println!("App Initialized.");
    }

    let arc = Arc::clone(&state.0);
    thread::spawn(move || loop {
        let lock = arc.lock(ThreadPriority::Main.into());
        // Handle errors, unwrap if you want
        if lock.is_err() {
            continue;
        }
        let lock = match lock {
            Ok(x) => x,
            Err(_) => continue,
        };

        // Here's your state
        let mut state = lock;

        // use it however you want, you can emit an event to FE as well.
        os::app::update(&mut state);
    });

    Ok(())
}

fn main() {
    let app_state: AppState = AppState::init();

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![init, run_macro])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
