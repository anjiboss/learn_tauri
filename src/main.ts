import { invoke } from "@tauri-apps/api/tauri";
import {
  isRegistered,
  register,
  unregister,
} from "@tauri-apps/api/globalShortcut";

let greetInputEl: HTMLInputElement | null;
let greetMsgEl: HTMLElement | null;
// let counterElm: HTMLElement | null;

// registerNewCmd();

async function greet() {
  if (greetMsgEl && greetInputEl) {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsgEl.textContent = await invoke("greet", {
      name: greetInputEl.value,
    });
  }
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  // counterElm = document.querySelector("#counter");

  document
    .querySelector("#greet-button")
    ?.addEventListener("click", () => greet());
});

async function registerNewCmd() {
  const registered = await isRegistered("CommandOrControl+Shift+U");
  const registering = async () => {
    await register("CommandOrControl+Shift+U", async () => {
      // const count: number = await invoke("count_many", { times: 5 });
      // if (counterElm) {
      //   counterElm.textContent = count.toString();
      // }
      invoke("open_docs");
    });
  };
  if (!registered) {
    console.log("Registering shortcut");
    registering();
  } else {
    // unregistering command
    invoke("log_console", { phrase: "Unregistering shortcut" });
    await unregister("CommandOrControl+Shift+U");
    invoke("log_console", { str: "Reigster again" });
    registering();
  }
}
