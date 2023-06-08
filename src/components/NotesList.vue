<template>
  <div>
    <ul>
      <li v-for="note in list" @click="handleNoteClick(note)">
        {{ note.title }}
      </li>
    </ul>
  </div>
</template>

<script lang="ts" setup>
import { invoke } from "@tauri-apps/api/tauri";
import { reactive, onBeforeMount } from 'vue';
import { Note, NoteStorage } from '../utils/utils';
import { store } from '../utils/store';

const list: Note[] = reactive([]);

async function requestNotesLits() {
  const data: NoteStorage = await invoke("get_all_notes");
  const parsed = Object.values(data).map(v => v);
  list.push(...parsed);
}

function handleNoteClick(note: Note) {
  store.noteId = note.id;
}

onBeforeMount(() => {
  requestNotesLits();
})

</script>

<style></style>
