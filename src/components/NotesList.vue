<template>
  <div>
    <FilteredTags @handle-clear-filters="handleClearFilters" />
    <ul>
      <template v-for="note in state.notes">
        <li
          :id="note.id"
          :class="{ 'active-note': store.note?.id === note.id }"
          @click="handleNoteClick(note)"
        >
          <a>
            {{ note.title }}
          </a>
        </li>
      </template>
    </ul>
  </div>
</template>

<script lang="ts" setup>
import FilteredTags from "./FilteredTags.vue";
import { useRouter } from "vue-router";
import { invoke } from "@tauri-apps/api/tauri";
import { reactive, onBeforeMount, watch, ref } from "vue";
import { Note, NoteStorage } from "../utils/utils";
import { store } from "../utils/store";

interface IState {
  notes: Note[];
  isActive: boolean;
}

const state: IState = reactive({
  notes: [],
  isActive: false,
});
const router = useRouter();

function parseAndSaveNotesToStore(data: NoteStorage) {
  const parsed = Object.values(data).map((v) => v);
  state.notes.length = 0;
  state.notes.push(...parsed);
  store.notes = parsed;
}

async function requestNotesList() {
  const data: NoteStorage = await invoke("get_all_notes");

  parseAndSaveNotesToStore(data);
  return true;
}

function handleNoteClick(note: Note) {
  store.note = note;
  router.push({
    path: `/editor/${note.id}`,
  });
}

function handleClearFilters() {
  store.filteredTags = [];
  requestNotesList();
}

function eqxcludeNotesBasedOnAppliedFilter(filteredTags: string[]) {
  const filtered = state.notes.filter((n) => {
    const t = n.tags.some((el) => filteredTags.includes(el));
    return t;
  });
  state.notes = filtered;
}

onBeforeMount(() => {
  requestNotesList();
});

watch(
  () => store.lastUpdate,
  async (nextValue, prevValue) => {
    if (nextValue !== prevValue) {
      await requestNotesList();
      if (store.filteredTags.length > 0) {
        eqxcludeNotesBasedOnAppliedFilter(store.filteredTags);
      }
    }
  }
);

watch(
  () => store.filteredTags,
  (newFilteredTags) => {
    eqxcludeNotesBasedOnAppliedFilter(newFilteredTags);
  },
  {
    deep: true,
  }
);
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

.drop {
  height: 100px;
  border: 1px solid salmon;
}
</style>
