import { invoke } from "@tauri-apps/api/tauri";
import { appWindow } from "@tauri-apps/api/window";
import { Command } from "@tauri-apps/api/shell";

window.addEventListener("DOMContentLoaded", app);

async function app() {
  await appWindow.onFocusChanged((event) => {
    console.log(event);
    if (!event.payload) {
      invoke("close_window", { windowLable: event.windowLabel });
    }

    const testBtn = document.getElementById("testBtn");
    testBtn?.addEventListener("click", () => {
      console.log("cicked");
      new Command("open_app", [
        "-b",
        "com.apple.systempreferences",
        "/System/Library/PreferencePanes/Security.prefPane",
      ]).spawn();
    });
  });
}
