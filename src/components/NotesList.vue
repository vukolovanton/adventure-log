<template>
  <div>
    <FilteredTags @handle-clear-filters="handleClearFilters" />

    <template v-for="(value, name) in state.notesWithFolders">
      <details open>
        <summary>{{ name }}</summary>
        <div
          :id="name.toString()"
          @drop.prevent="onDrop"
          @dragenter.prevent
          @dragover.prevent
          class="drop"
        >
          <ul>
            <template v-for="note in value">
              <li
                @dragstart="(event) => onDragStart(event, note.id)"
                :id="note.id"
                draggable="true"
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
      </details>
    </template>

    <ul>
      <template v-for="note in state.notes">
        <li
          v-if="note.folder_id === ''"
          @dragstart="(event) => onDragStart(event, note.id)"
          :id="note.id"
          draggable="true"
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

interface NotesFolder {
  [key: string]: Note[];
}

interface IState {
  notes: Note[];
  notesWithFolders: NotesFolder;
  isActive: boolean;
}

const draggedNoteId = ref("");

const state: IState = reactive({
  notes: [],
  notesWithFolders: {},
  isActive: false,
});
const router = useRouter();

async function onDrop(event: DragEvent) {
  const folderId = (event.currentTarget as HTMLElement).id;

  const updatedNotes: NoteStorage = await invoke("update_note_folder_id", {
    noteId: draggedNoteId.value,
    folderId: folderId,
  });

  parseAndSaveNotesToStore(updatedNotes);
  groupNotesByFolder(state.notes);
}

function onDragStart(event: DragEvent, id: string) {
  draggedNoteId.value = id;
}

function groupNotesByFolder(notes: Note[]) {
  const groupedByFolders: NotesFolder = {};
  notes.forEach((note) => {
    if (note.folder_id !== "") {
      const folderId = note.folder_id.trim();
      const isFolderArrayExists = groupedByFolders[folderId];
      if (isFolderArrayExists) {
        groupedByFolders[folderId].push(note);
      } else {
        groupedByFolders[folderId] = [];
        groupedByFolders[folderId].push(note);
      }
    }
  });

  state.notesWithFolders = groupedByFolders;
}

function parseAndSaveNotesToStore(data: NoteStorage) {
  const parsed = Object.values(data).map((v) => v);
  state.notes.length = 0;
  state.notes.push(...parsed);
  store.notes = parsed;
}

async function requestNotesList() {
  const data: NoteStorage = await invoke("get_all_notes");

  parseAndSaveNotesToStore(data);
  groupNotesByFolder(state.notes);

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
