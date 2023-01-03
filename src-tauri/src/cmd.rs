use std::sync::{Arc, Mutex};

use tauri::{command, State};

#[derive(Default)]
pub struct Counter(pub Arc<Mutex<i32>>);

#[command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[command]
pub fn log_console(phrase: &str) {
    println!("{}", phrase);
}

#[command]
pub fn count_many(times: i32, counter: State<'_, Counter>) -> i32 {
    let mut value = counter.0.lock().unwrap();
    *value += times;
    println!("Counter: {}", *value);
    return *value;
}

#[command]
pub async fn open_docs(handle: tauri::AppHandle) {
    tauri::WindowBuilder::new(
        &handle,
        "test", /* the unique window label */
        tauri::WindowUrl::External("https://tauri.app/".parse().unwrap()),
    )
    .build()
    .unwrap();
}

#[command]
pub fn close_window(window_lable: &str, _app: tauri::AppHandle, window: tauri::Window) {
    println!(
        "Requested to close the window: window_lable: {}, window: {}",
        window_lable,
        window.label()
    );
    window.hide().unwrap();
}
