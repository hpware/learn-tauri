// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name) // {} is %s %f %d etc.. in c++
}

#[tauri::command]
fn test_command(pass: &str) -> String {
    // no case on rust
    format!("hello test command! {}", pass) // no ; in rust ig (nvm its not)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, test_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
