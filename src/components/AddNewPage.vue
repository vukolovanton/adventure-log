<template>
  <div class="controls">
    <button aria-label="Add new note" @click="handleAddNew" class="main">
      Add New
    </button>
    <button aria-label="Settings" @click="handleGoHome" class="settings">
      <Gear />
    </button>
  </div>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import Gear from "./icons/Gear.vue";
import { useRouter } from "vue-router";
import { store } from "../utils/store";

const router = useRouter();

async function handleSave() {
  const id = Date.now().toString();
  const note = {
    id,
    title: "New",
    description: "",
    tags: [],
    folder: "",
  };
  await invoke("save_note", {
    ...note,
  });

  store.lastUpdate = Date.now();
  return note;
}

async function handleAddNew() {
  const note = await handleSave();
  store.note = note;
  store.notes.push(note);
  router.push(`/editor/${note.id}`);
}

function handleGoHome() {
  router.push(`/`);
}
</script>

<style scoped>
.controls {
  display: flex;
  align-items: center;
  justify-content: space-between;
  flex-direction: row;
  margin-bottom: 0.5rem;
}
</style>
