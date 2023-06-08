<template>
  <div class="Editor">
    <span>{{ store.noteId }}</span>
    <button @click="handleSave">Save</button>
    <input v-model="title" />
    <textarea class="container" v-model="description">

  </textarea>
  </div>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { onUpdated, ref } from 'vue';
import { store } from '../utils/store';

const description = ref("");
const title = ref("");

async function handleSave() {
  await invoke("save_note", {
    id: Date.now().toString(),
    title: title.value,
    description: description.value,
    tags: [],
  })
}

onUpdated(() => {
  console.log(store)
})

</script>

<style scoped>
.container {
  width: 100%;
  height: 100%;
  border: 1px solid grey;
}


.Editor {
  grid-area: Editor;
}
</style>
