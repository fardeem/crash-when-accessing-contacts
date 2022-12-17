import { invoke } from "@tauri-apps/api/tauri";

window.addEventListener("DOMContentLoaded", () => {
  document.getElementById('request-permission')?.addEventListener('click', () => {
    invoke('request_permission')
  })
});
