// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

use tauri::State;
use windows::Win32::{
    Foundation::HWND,
    UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowTextA},
};

struct AppState {
    win_last: Mutex<HWND>,
    win_last_name: Mutex<String>,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn macro_undo(app: State<AppState>) -> Result<(String, String), ()> {
    let mut win_last = app.win_last.lock().unwrap();
    let mut win_last_name = app.win_last_name.lock().unwrap();
    let last_name = win_last_name.clone();

    unsafe {
        let win: HWND;
        let lpstring = &mut vec![0; 255].into_boxed_slice();
        let name: String;

        win = GetForegroundWindow();
        name = GetWindowTextA(win, lpstring).to_string();

        *win_last = win;
        *win_last_name = name.clone();

        return Ok((name, last_name));
    };
}

fn main() {
    let app = AppState {
        win_last: Mutex::new(HWND(0)),
        win_last_name: Mutex::new("".to_owned()),
    };
    tauri::Builder::default()
        .manage(app)
        .invoke_handler(tauri::generate_handler![macro_undo])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
