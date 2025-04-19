import App from "./App.vue";

import { createApp } from "vue";
import { invoke } from "@tauri-apps/api/core";

createApp(App).mount("#app");

onload = async () => {
  await invoke("show_window");
};
