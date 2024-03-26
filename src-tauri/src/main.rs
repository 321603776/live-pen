// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use enigo::{Enigo, MouseControllable};
use xcap::Monitor;
use tauri::Manager;

static mut SCREENSHOT_PATH: String = String::new();

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn capture_screen() -> String {
    let enigo = Enigo::new();
    let cursor_location = enigo.mouse_location();
    let monitor = Monitor::from_point(cursor_location.0, cursor_location.1).expect("获取屏幕失败");
    let image_buffer = monitor.capture_image().expect("捕获屏幕失败");
    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join("screenshot.png");
    let _ = image_buffer.save(&file_path);
    file_path.to_str().unwrap().to_owned()
}

#[tauri::command]
fn update_screenshot() -> String {
    unsafe {
        SCREENSHOT_PATH = capture_screen();
        SCREENSHOT_PATH.clone()
    }
}

#[tauri::command]
fn get_screenshot_path() -> String {
    unsafe {
        SCREENSHOT_PATH.clone()
    }
}

fn make_tray() -> tauri::SystemTray {
    let tray_menu = tauri::SystemTrayMenu::new()
        .add_item(tauri::CustomMenuItem::new("quit".to_string(), "Quit"));

    return tauri::SystemTray::new()
        .with_menu(tray_menu);
}

fn handle_tray_event(app: &tauri::AppHandle, event: tauri::SystemTrayEvent) {
    if let tauri::SystemTrayEvent::MenuItemClick { id, .. } = event {
        if id.as_str() == "quit" {
            app.exit(0);
        }
    }
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }
            Ok(())
        })
        .system_tray(make_tray())
        .on_system_tray_event(handle_tray_event)
        .invoke_handler(tauri::generate_handler![greet, update_screenshot, get_screenshot_path])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
