import { invoke } from "@tauri-apps/api/tauri";
import { appWindow } from "@tauri-apps/api/window";
import { Command } from "@tauri-apps/api/shell";
import { Apps } from "./types";

window.addEventListener("DOMContentLoaded", app);

let contents: Apps | null = null;
let appContainer: HTMLDivElement | null = null;

async function app() {
  contents = await invoke("send_context");
  if (contents) {
    appContainer = document.getElementById("app_container") as HTMLDivElement;
    addAppsToContainer(appContainer, contents);
  }

  await appWindow.onFocusChanged((event) => {
    console.log(event);
    if (!event.payload) {
      invoke("close_window", { windowLable: event.windowLabel });
    }
  });
}

function addAppsToContainer(container: HTMLDivElement, apps: Apps) {
  apps.SPApplicationsDataType.forEach((app, index) => {
    const appDiv = document.createElement("div");
    appDiv.classList.add("app");
    appDiv.innerHTML = app._name;
    const indexAttr = document.createAttribute("index");
    indexAttr.value = index.toString();
    appDiv.attributes.setNamedItem(indexAttr);
    appDiv.onclick = () => {
      new Command("open_app", [app.path]).spawn();
    };
    container.appendChild(appDiv);
  });
}
