// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    process::Child,
    sync::{Arc, Mutex, MutexGuard},
    thread,
};

use tauri::{window, Manager, Runtime, State};
use windows::Win32::{
    Foundation::HWND,
    UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowTextA, GetWindowThreadProcessId},
};

pub struct App {
    app_hwnd: HWND,
    hwnd: HWND,
    hwnd_prev: HWND,
    initialized: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            app_hwnd: HWND(0),
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
    let mut init_lock = state.0.lock().unwrap();
    if init_lock.initialized {
        return Err("App already initialized.".to_owned());
    } else {
        init_lock.app_hwnd = HWND(window.hwnd().unwrap().0);
        init_lock.initialized = true;
    }

    let arc = Arc::clone(&state.0);
    thread::spawn(move || loop {
        let lock = arc.try_lock();
        // Handle errors, unwrap if you want
        if lock.is_err() {
            break;
        }
        let lock = match lock {
            Ok(x) => x,
            Err(_) => break,
        };

        // Here's your state
        let mut state = lock;

        // use it however you want, you can emit an event to FE as well.
        runtime(&mut state)
    });

    Ok(())
}

fn runtime(state: &mut MutexGuard<App>) {
    state.app_hwnd = unsafe { HWND(0) };

    update_hwnd(state);

    println!(
        "[runtime]: app_hwnd: {}, hwnd: {}",
        state.app_hwnd.0, state.hwnd.0
    );
}

fn update_hwnd(state: &mut MutexGuard<App>) {
    state.hwnd_prev = state.hwnd;
    state.hwnd = unsafe { GetForegroundWindow() };
}

fn main() {
    let app_state: AppState = AppState::init();

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![init, macro_undo])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
