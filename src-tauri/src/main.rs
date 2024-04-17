// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod platforms;

use crate::platforms::app::{update, App};

use std::{
    sync::{Arc, Mutex},
    thread,
};

use platforms::app::PlatApp;
use tauri::{Runtime, State};

pub struct AppState(pub Arc<Mutex<PlatApp>>);
impl AppState {
    pub fn init() -> Self {
        AppState(Arc::new(Mutex::new(PlatApp::new())))
    }
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn run_macro(_name: String, _state: State<AppState>) -> Result<(), ()> {
    Ok(())
}

#[tauri::command]
// https://github.com/tauri-apps/tauri/discussions/4775
fn init<R: Runtime>(_window: tauri::Window<R>, state: State<'_, AppState>) -> Result<(), String> {
    let mut init_lock = state.0.lock().unwrap();
    if init_lock.initialized {
        return Err("App already initialized.".to_owned());
    } else {
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
        update(&mut state);
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
