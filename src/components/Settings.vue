<template>
  <section>
    <h1>Adventure Log</h1>

    <div class="setting">
      <div class="setting-info">
        <h3 class="setting-title">Application data</h3>
        <p class="setting-description">
          Here you can export and import application data in case you need a
          backup
        </p>
      </div>
      <div class="setting-actions">
        <button @click="handleExport">Export</button>
        <div>
          <label for="fileInput">Import</label>
          <input
            id="fileInput"
            type="file"
            multiple="false"
            accept="application/json"
            @change="handleImport"
            name="Import"
          />
        </div>
      </div>
    </div>
  </section>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { writeTextFile, BaseDirectory } from "@tauri-apps/api/fs";
import { NoteStorage } from "../utils/interfaces";

async function handleExport() {
  const data: NoteStorage = await invoke("get_all_notes");
  const stringifiedData = JSON.stringify(data);

  await writeTextFile("data.json", stringifiedData, {
    dir: BaseDirectory.Desktop,
  });
}

async function handleImport(event: Event) {
  if (event.target) {
    const files: FileList | null = (event.target as HTMLInputElement).files;
    if (files) {
      const file = files[0];
      if (file.type !== "application/json") return;
      const reader = new FileReader();
      reader.onload = (event) => {
        if (event && event.target) {
          const content = event.target.result;
          console.log(content);
        }
      };
      reader.onloadend = () => {
        const input: HTMLElement | null = document.getElementById("fileInput");
        if (input) {
          (input as HTMLInputElement).value = "";
        }
      };
      reader.readAsText(file);
    }
  }
}
</script>

<style scoped>
section {
  display: flex;
  flex-direction: column;
  gap: 2rem;
}

input[type="file"] {
  color: transparent;
}

.setting {
  display: flex;
  justify-content: space-between;
  gap: 2rem;
}
.setting-info {
  display: flex;
  flex-direction: column;
  flex: 3;
}

.setting-description {
  opacity: 0.7;
}

.setting-actions {
  flex: 2;
  gap: 0.5rem;
  align-items: flex-start;
}
</style>
