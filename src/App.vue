<script setup lang="ts">
import { disableKeyboardShortcuts, disableFunctionKeys, disableMouseButtons } from "./disable-inputs";

import { platform } from "@tauri-apps/plugin-os";

disableKeyboardShortcuts();
disableFunctionKeys();
disableMouseButtons();

// We've set the background color of client area to transparent to apply window vibrancy effect.
// On MacOS and Windows, this area can be correctly handled by their DWM, which will
// apply another translucent background for it, i.e., the Mica material on Windows 11.
// So, the actual effect looks like that, the background is still there, but its
// color and transparency has been modified.
// Unfortunately, this effect is only available on that two platforms, and will lead to some
// unexpected problems when running on other platforms!
// For example, if you open this program on Linux, you'll still see a transparent client area,
// because the desktop environment varys from different Linux distributions, and this caused that
// we cannot find a uniform method to apply window vibrancy for client area, like what MacOS and Windows does.
// Thus, we have to dynamically set the backgroud color to white to avoid such these problems.
if (!["macos", "windows"].includes(platform())) {
  document.body.style.backgroundColor = "white";
}
</script>

<template>
</template>

<style>
:root {
  user-select: none;

  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: transparent;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: transparent;
  }
}
</style>