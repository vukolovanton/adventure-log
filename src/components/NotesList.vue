<template>
  <div>
    <div>
      <span v-for="filter in store.filteredTags">#{{ filter }}</span>
      <button v-if="store.filteredTags.length > 0" @click="handleClearFilters">Clear filters</button>
    </div>
    <ul>
      <li v-for="note in state.notes" :class="{ 'active-note': store.note?.id === note.id }">
        <a @click="handleNoteClick(note)">
          {{ note.title }}
        </a>
        <button @click="handleDeleteNote(note)">X</button>
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

interface IState {
  notes: Note[],
  isActive: boolean,
}
const state: IState = reactive({
  notes: [],
  isActive: false,
})
const router = useRouter();

async function requestNotesList() {
  const data: NoteStorage = await invoke("get_all_notes");
  const parsed = Object.values(data).map(v => v);
  state.notes.length = 0;
  state.notes.push(...parsed)
  store.notes = parsed;
}

function handleNoteClick(note: Note) {
  store.note = note;
  router.push({
    path: `/editor/${note.id}`,
  });
}

function handleClearFilters() {
  store.filteredTags = [];
  requestNotesList()
}

async function handleDeleteNote(note: Note) {
  await invoke("delete_note", {
    id: note.id
  });
  store.lastUpdate = Date.now();
}

onBeforeMount(() => {
  requestNotesList();
})

watch(() => store.lastUpdate, (nextValue, prevValue) => {
  if (nextValue !== prevValue) {
    requestNotesList()
  }
})

watch(() => store.filteredTags, filteredTags => {
  const filtered = state.notes.filter(n => {
    const t = n.tags.some(el => filteredTags.includes(el));
    return t;
  });
  state.notes = filtered;
},
  {
    deep: true
  })
</script>

<style scoped>
li {
  cursor: pointer;
}

a:hover {
  background-color: palegreen;
}

.active-note {
  font-weight: bold;
}
</style>
