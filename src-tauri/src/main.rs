#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod cmd;
mod config;
use config::macos_apps;
use std::time::Instant;
use tauri::{GlobalShortcutManager, Manager};

// #[derive(Clone, serde::Serialize)]
// struct Payload {
//     apps: config::macos_apps::MacApps,
// }

#[tauri::command]
fn send_context(_app_handle: tauri::AppHandle) -> macos_apps::MacApps {
    let start = Instant::now();
    let apps = config::get_all_applications();
    println!("triggered {}", apps.sp_applications_data_type[0].name);
    println!(
        "Time elapsed in expensive_function() is: {:?}",
        start.elapsed()
    );
    apps.into()
}

fn main() {
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
            cmd::close_window,
            send_context
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|app_handle, event| match event {
            tauri::RunEvent::Ready => {
                let app_handle = app_handle.clone();
                // register shortcuts
                app_handle
                    .global_shortcut_manager()
                    .register("CommandOrControl+Shift+U", move || {
                        for (title, window) in app_handle.windows() {
                            println!("{}", title);
                            window.show().unwrap();
                            window.center().unwrap();
                            window.set_focus().unwrap();
                        }
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
