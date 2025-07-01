import { invoke } from "@tauri-apps/api/core";
import { listen, TauriEvent } from '@tauri-apps/api/event';
import { useSplashScreen } from "./useSplashScreen";
import { HISTORY_FILES_KEY, store } from "./useStore";

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

export interface FileInfo {
  "accessed": number,
  "created": number,
  "ext": string,
  "file_type": string,
  "formatted_size": string,
  "is_dir": boolean,
  "is_file": boolean,
  "modified": number,
  "name": string,
  "path": string,
  "processing_result": string,
  "size": number
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

  await invoke<FileInfo>("handle_dropfile", {
    path: paths[0],
  }).then(async (response) => {
    console.log("Response from handle_dropfile:", response);

    const historyFiles = await store.get<FileInfo[]>(HISTORY_FILES_KEY);

    historyFiles?.unshift(response);

    const uniqueFiles = Array.from(new Set(historyFiles?.map(file => file.path)))
      .map(path => historyFiles?.find(file => file.path === path));

    await store.set(HISTORY_FILES_KEY, uniqueFiles);

    files = await store.get<FileInfo[]>(HISTORY_FILES_KEY);
  })
});

let files = await store.get<FileInfo[]>(HISTORY_FILES_KEY);
console.log("Files from store:", files);
