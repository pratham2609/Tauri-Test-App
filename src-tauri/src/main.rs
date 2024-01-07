// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
// #[tauri::command] attribute macro that allows your function to communicate with the JavaScript context.

// Lastly, we also need to tell Tauri about our newly created command so that it can route calls accordingly.
// This is done with the combination of the .invoke_handler() function and the generate_handler![] macro you can see below:
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
