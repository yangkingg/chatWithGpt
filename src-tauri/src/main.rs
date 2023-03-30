// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use http::get_message;

mod http;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_message])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
