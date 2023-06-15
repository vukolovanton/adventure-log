<template>
  <div>
    <div class="applied-filters">
      <span v-for="filter in store.filteredTags">#{{ filter }}</span>
      <button v-if="store.filteredTags.length > 0" @click="handleClearFilters">
        <Xmark />
      </button>
    </div>
    <ul>
      <li v-for="note in state.notes" :class="{ 'active-note': store.note?.id === note.id }"
        @click="handleNoteClick(note)">
        <a>
          {{ note.title }}
        </a>
      </li>
    </ul>
  </div>
</template>

<script lang="ts" setup>
import Xmark from './icons/Xmark.vue';
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
  padding: 0.1rem;
  opacity: 0.6;
}

li:hover {
  text-decoration: underline;
  text-decoration-color: cadetblue;
}

.active-note {
  text-decoration-line: underline;
  opacity: 1;
}

ul {
  max-height: 90vh;
  overflow: scroll;
  list-style-type: none;
  margin: 0;
  padding: 0;
}

.applied-filters {
  display: flex;
  gap: 0.3rem;
  flex-wrap: wrap;
}
</style>
