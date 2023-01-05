use home::home_dir;
use std::fs::{create_dir_all, write, File};
use std::io::BufReader;
use std::path::Path;
use std::process::Command;
pub mod macos_apps;

#[cfg(target_os = "macos")]
pub fn get_all_applications() -> macos_apps::MacApps {
    let user_home = home_dir().unwrap();
    let user_home = user_home.to_str().unwrap();
    let cached_dir: String = String::from(user_home) + "/.cache/taxuri";
    let cached_apps: String = String::from(user_home) + "/.cache/taxuri/apps.json";
    if !Path::new(&cached_apps).exists() {
        create_dir_all(&cached_dir).unwrap();
        let output = Command::new("system_profiler")
            .arg("-json")
            .arg("SPApplicationsDataType")
            .current_dir(&cached_dir)
            .output()
            .unwrap();
        // save output to file
        write(cached_apps, &output.stdout).expect("Write error.");
        let apps: macos_apps::MacApps = serde_json::from_slice(&output.stdout).expect("JSON error");
        return apps;
    }

    let file = File::open(&cached_apps).expect("File not exists");
    let reader = BufReader::new(file);

    let apps: macos_apps::MacApps = serde_json::from_reader(reader).expect("read json failed");
    apps
}

#[cfg(target_os = "window")]
pub fn get_apps() {
    println!("this is window");
}
