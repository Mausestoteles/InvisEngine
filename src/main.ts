import { invoke } from "@tauri-apps/api/core";

const status = document.querySelector<HTMLElement>("#status")!;
const error = document.querySelector<HTMLElement>("#error")!;

function showStatus(protectedFromCapture: boolean) {
  status.textContent = protectedFromCapture ? "AN (unsichtbar für Capture)" : "AUS (sichtbar für Capture)";
  status.className = protectedFromCapture ? "on" : "off";
}

async function run(action: () => Promise<void>) {
  error.textContent = "";
  try {
    await action();
  } catch (e) {
    error.textContent = String(e);
  }
}

document.querySelector("#toggle")!.addEventListener("click", () =>
  run(async () => {
    const current = await invoke<boolean>("get_capture_protection");
    await invoke("set_capture_protection", { enabled: !current });
    showStatus(await invoke<boolean>("get_capture_protection"));
  }),
);

document.querySelector("#browser")!.addEventListener("click", () =>
  run(() => invoke("open_test_browser")),
);

run(async () => showStatus(await invoke<boolean>("get_capture_protection")));
