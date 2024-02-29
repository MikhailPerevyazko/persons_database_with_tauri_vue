<script setup lang="ts">
import { save } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api/tauri";
async function openSaveDialog() {
  const filePath = await save({
    filters: [
      {
        name: "Yaml",
        extensions: ["yml", "yaml"],
      },
    ],
  });
  console.log("Путь к файлу: " + filePath);
  invoke("open_db", { filePath: filePath });
}
</script>

<template>
  <div>
    <el-button type="primary" @click="openSaveDialog">Select a file</el-button>
  </div>
</template>

<style scoped></style>
