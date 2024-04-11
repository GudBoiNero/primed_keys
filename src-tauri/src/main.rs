// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use tauri::{
    plugin::{Builder as PluginBuilder, TauriPlugin},
    Runtime, State,
};
use windows::Win32::{
    Foundation::HWND,
    UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowTextA},
};

pub struct App {
    hwnd: HWND,
    hwnd_prev: HWND,
    initialized: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            hwnd: HWND(0),
            hwnd_prev: HWND(0),
            initialized: false,
        }
    }
}

pub struct AppState(pub Arc<Mutex<App>>);
impl AppState {
    pub fn init() -> Self {
        AppState(Arc::new(Mutex::new(App::new())))
    }
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn macro_undo(state: State<AppState>) -> Result<(), ()> {
    Ok(())
}

#[tauri::command]
// https://github.com/tauri-apps/tauri/discussions/4775
fn init<R: Runtime>(window: tauri::Window<R>, state: State<'_, AppState>) -> Result<(), String> {
    if let mut lock = state.0.lock().unwrap() {
        if lock.initialized {
            return Err("App already initialized.".to_owned());
        } else {
            lock.initialized = true;
        }
    }

    let state = Arc::clone(&state.0);

    thread::spawn(move || loop {
        let lock = state.try_lock();
        // Handle errors, unwrap if you want
        if lock.is_err() {
            break;
        }
        let lock = match lock {
            Ok(x) => x,
            Err(_) => break,
        };

        // Here's your state
        let state = Box::new(lock).as_mut();

        // use it however you want, you can emit an event to FE as well.
        thread::sleep(Duration::from_millis(1000));
        print!("Loop\n")
    });

    Ok(())
}

fn main() {
    let app_state: AppState = AppState::init();

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![init, macro_undo])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
