#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::{Arc, Mutex};

use tauri::{AppHandle, GlobalShortcutManager, State};

#[derive(Default)]
struct Counter(Arc<Mutex<i32>>);

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn log_console(phrase: &str) {
    println!("{}", phrase);
}

#[tauri::command]
fn count_many(times: i32, counter: State<'_, Counter>) -> i32 {
    let mut value = counter.0.lock().unwrap();
    *value += times;
    println!("Counter: {}", *value);
    return *value;
}

#[tauri::command]
async fn open_docs(handle: tauri::AppHandle) {
    tauri::WindowBuilder::new(
        &handle,
        "test", /* the unique window label */
        tauri::WindowUrl::External("https://tauri.app/".parse().unwrap()),
    )
    .build()
    .unwrap();
}

fn create_new_window(handle: &AppHandle) {
    tauri::WindowBuilder::new(
        handle,
        "test", /* the unique window label */
        tauri::WindowUrl::App("launcher.html".into()),
    )
    .always_on_top(true)
    .center()
    .inner_size(500.0, 300.0)
    .decorations(false)
    .resizable(false)
    .build()
    .unwrap();
}

fn main() {
    tauri::Builder::default()
        .manage(Counter(Default::default()))
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { .. } => {
                event.window().hide().unwrap();
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            log_console,
            count_many,
            open_docs
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        // .expect("error while trying to hide application")
        .run(|app_handle, event| match event {
            tauri::RunEvent::Ready => {
                let app_handle = app_handle.clone();
                // register shortcuts
                app_handle
                    .global_shortcut_manager()
                    .register("CommandOrControl+Shift+U", move || {
                        println!("triggered");
                        create_new_window(&app_handle);
                    })
                    .unwrap();
            }
            // run in background
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        })
}
