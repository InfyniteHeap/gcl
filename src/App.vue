<script setup lang="ts">
import { disableFunctionKeys, disableMouseButtons } from "./disable-inputs";

disableFunctionKeys();
disableMouseButtons();
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