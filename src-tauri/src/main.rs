#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod cmd;
use tauri::{GlobalShortcutManager, Manager};

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
            cmd::close_window
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
                        for (title, window) in app_handle.windows() {
                            println!("{}", title);
                            window.show().unwrap();
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
