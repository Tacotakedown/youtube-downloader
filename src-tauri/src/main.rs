// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use window_shadows::set_shadow;
use window_vibrancy::apply_mica;

mod config;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            config::read_config,
            config::write_config
        ])
        .setup(|app| {
            let window = app.get_window("main").unwrap();

            #[cfg(target_os = "windows")]
            apply_mica(&window, Some(true)).unwrap_or_default();
            set_shadow(&window, true).unwrap_or_default();

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
