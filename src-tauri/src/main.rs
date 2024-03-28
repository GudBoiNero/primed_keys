// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

use tauri::{Manager, State};
use windows::Win32::{
    Foundation::HWND,
    UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowTextA},
};

pub struct App {}

impl App {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct AppState(pub Mutex<App>);

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn macro_undo(app_state: State<App>) -> Result<(), ()> {
    Ok(())
}

fn main() {
    let app_state: AppState = AppState(Mutex::new(App::new()));

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![macro_undo])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
