<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const greetMsg = ref("");
async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg.value = await invoke("greet", { name: name.value });
}
</script>

<template>
  <main class="container">
    <h1>CVToolKit</h1>
    <img src="/public/CV_logo.png" alt="ToolKit logo" class=" CV logo" />

    <div class="row">

    </div>

  </main>
</template>

<style>
.CV.logo
</style>