// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(target_os = "linux")]
pub(crate) mod linux;
#[cfg(target_os = "macos")]
pub(crate) mod macos;
#[cfg(windows)]
pub(crate) mod windows;

#[cfg(target_os = "linux")]
use linux as os;
#[cfg(target_os = "macos")]
use macos as os;
#[cfg(windows)]
use windows as os;

mod app;

use crate::app::App;
use os::app::OSApp;
use std::{
    sync::{Arc, Mutex},
    thread,
};
use tauri::{utils::config::WindowsConfig, Runtime, State};

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
    let arc = Arc::clone(&_state.0);
    let lock = arc.try_lock();
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
        os::app::update(&mut state);
    });

    Ok(())
}

fn main() {
    let app_state: AppState = AppState::init();

    let windows_attributes = tauri_build::WindowsAttributes::new().app_manifest(
        r#"
    <assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
      <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
          <security>
              <requestedPrivileges>
                  <requestedExecutionLevel level="requireAdministrator" uiAccess="false" />
              </requestedPrivileges>
          </security>
      </trustInfo>
    </assembly>
    "#,
    );
    let attributes = tauri_build::Attributes::new().windows_attributes(windows_attributes);
    tauri_build::try_build(attributes).unwrap();

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![init, run_macro])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
