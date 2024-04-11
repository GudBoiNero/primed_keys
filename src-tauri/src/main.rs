// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

use tauri::{
    plugin::{Builder as PluginBuilder, TauriPlugin},
    Manager, Runtime, State,
};
use windows::Win32::{
    Foundation::HWND,
    UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowTextA},
};

pub struct App {
    hwnd: HWND,
    hwnd_prev: HWND,
}

impl App {
    pub fn new() -> Self {
        Self {
            hwnd: HWND(0),
            hwnd_prev: HWND(0),
        }
    }
}

pub struct AppState(pub Mutex<App>);

pub struct AppManager;
impl AppManager {
    pub fn init<R: Runtime>() -> TauriPlugin<R> {
        PluginBuilder::new("process")
            .on_event(|app, event| match event {
                tauri::RunEvent::MainEventsCleared => {}
                _ => {}
            })
            .build()
    }
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn macro_undo(app_state: State<App>) -> Result<(), ()> {
    Ok(())
}

fn main() {
    let app_state: AppState = AppState(Mutex::new(App::new()));

    tauri::Builder::default()
        .manage(app_state)
        .plugin(AppManager::init())
        .invoke_handler(tauri::generate_handler![macro_undo])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
