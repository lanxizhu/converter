import { invoke } from "@tauri-apps/api/core";
import { listen, TauriEvent } from '@tauri-apps/api/event';
import { useSplashScreen } from "./useSplashScreen";

let greetInputEl: HTMLInputElement | null;
let greetMsgEl: HTMLElement | null;

async function greet() {
  if (greetMsgEl && greetInputEl) {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsgEl.textContent = await invoke("greet", {
      name: greetInputEl.value,
    });
  }
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-form")?.addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });
});

useSplashScreen();

const areaEl = document.getElementById("drop-area");

type DragEventPayload = {
  paths: string[];
  position: {
    x: number;
    y: number;
  };
}

listen<DragEventPayload>(TauriEvent.DRAG_DROP, async (event) => {
  const clientRect = areaEl?.getBoundingClientRect();
  if (!clientRect) return;

  const { left, top, width, height } = clientRect;

  const { payload } = event;

  const { paths, position } = payload;

  const inside = position.x >= left &&
    position.x <= left + width &&
    position.y >= top &&
    position.y <= top + height;


  if (!inside) {
    console.log("File dropped outside the drop area bounds.");
    return;
  }

  console.log("File paths:", paths);
  console.log("File dropped within the drop area bounds.");

  await invoke("handle_dropfile", {
    path: paths[0],
  }).then((response) => {
    console.log("Response from handle_dropfile:", response);
  })
});