<template>
  <div class="Editor">
    <button @click="handleSave">Save</button>
    <input v-model="state.title" />
    <textarea class="container" v-model="state.description" />
  </div>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { reactive, ref, watch } from 'vue';
import { useRoute } from 'vue-router'
import { store, Store } from '../utils/store';
import { Note } from "../utils/utils";

interface State {
  editedNote: Note | null,
  title: string,
  description: string,
}

// const description = ref("");
// const title = ref("");
const state: State = reactive({
  editedNote: null,
  title: "",
  description: "",
})

const route = useRoute()

async function handleSave() {
  await invoke("save_note", {
    id: store.note ? store.note.id : Date.now().toString(),
    title: state.title,
    description: state.description,
    tags: [],
  })

  store.lastUpdate = Date.now();
}

function findAndSetNote(newId: string) {
  const note = store.notes.find(note => note.id === newId);
  if (note) {
    state.editedNote = note;
    state.description = note.description;
    state.title = note.title;
  }
}

function setDefaultNote() {
  state.editedNote = null;
  state.description = ""
  state.title = ""
}

watch(
  () => route.params.id,
  async newId => {
    if (newId) {
      findAndSetNote(newId as string);
    } else {
      setDefaultNote();
    }
  }
)

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
