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
import { useRouter } from 'vue-router';
import { invoke } from "@tauri-apps/api/tauri";
import { reactive, onBeforeMount, watch } from 'vue';
import { Note, NoteStorage } from '../utils/utils';
import { store } from '../utils/store';

const list: Note[] = reactive([]);
const router = useRouter();

async function requestNotesList() {
  const data: NoteStorage = await invoke("get_all_notes");
  const parsed = Object.values(data).map(v => v);
  list.length = 0;
  list.push(...parsed)
  store.notes = parsed;
}

function handleNoteClick(note: Note) {
  store.note = note;
  router.push({
    path: `/editor/${note.id}`,
  });
}

onBeforeMount(() => {
  requestNotesList();
})

watch(() => store.lastUpdate, (nextValue, prevValue) => {
  if (nextValue !== prevValue) {
    requestNotesList()
  }
})
</script>

<style></style>
