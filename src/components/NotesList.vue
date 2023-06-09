<template>
  <div>
    <div>
      <span v-for="filter in store.filteredTags">#{{ filter }}</span>
      <button @click="handleClearFilters">Clear filters</button>
    </div>
    <ul>
      <li v-for="note in state.notes" @click="handleNoteClick(note)">
        <a>
          {{ note.title }}
        </a>
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
}
const state: IState = reactive({
  notes: [],
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
</style>
