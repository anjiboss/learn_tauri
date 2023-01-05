#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod cmd;
mod config;
use std::time::Instant;
use tauri::{GlobalShortcutManager, Manager};

#[derive(Clone, serde::Serialize)]
struct Payload {
    apps: config::macos_apps::MacApps,
}

fn main() {
    let start = Instant::now();
    let apps = config::get_all_applications();
    println!(
        "Time elapsed in expensive_function() is: {:?}",
        start.elapsed()
    );
    tauri::Builder::default()
        .manage(cmd::Counter(Default::default()))
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { .. } => {
                event.window().hide().unwrap();
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            cmd::greet,
            cmd::log_console,
            cmd::count_many,
            cmd::open_docs,
            cmd::close_window
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|app_handle, event| match event {
            tauri::RunEvent::Ready => {
                let app_handle = app_handle.clone();
                app_handle.emit_all("take_apps", Payload { apps: apps });
                // register shortcuts
                app_handle
                    .global_shortcut_manager()
                    .register("CommandOrControl+Shift+U", move || {
                        println!("triggered");
                        for (title, window) in app_handle.windows() {
                            println!("{}", title);
                            window.show().unwrap();
                            window.center().unwrap();
                            window.set_focus().unwrap();
                        }
                        // create_new_window(&app_handle);
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
