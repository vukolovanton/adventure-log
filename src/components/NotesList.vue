<template>
  <div>
    <FilteredTags @handle-clear-filters="handleClearFilters" />
    <button @click="handleCreateNewFolder">new folder</button>

    <template v-for="(value, folder) in state.notesWithFolders">
      <details open>
        <summary>
          {{ folder }}
        </summary>
        <div
          :data-folder="folder.toString()"
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
          v-if="note.folder === ''"
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
  draggedNoteId: string;
}

const state: IState = reactive({
  notes: [],
  notesWithFolders: {},
  isActive: false,
  draggedNoteId: "",
});
const router = useRouter();

function handleCreateNewFolder() {
  function getRandomInt() {
    return Math.floor(Math.random() * 100);
  }
  state.notesWithFolders[`folder_${getRandomInt()}`] = [];
}

async function onDrop(event: DragEvent) {
  const folder = (event.currentTarget as HTMLElement).getAttribute(
    "data-folder"
  );

  const updatedNotes: NoteStorage = await invoke("update_note_folder", {
    noteId: state.draggedNoteId,
    folder: folder,
  });

  parseAndSaveNotesToStore(updatedNotes);
  groupNotesByFolder(state.notes);
}

function onDragStart(event: DragEvent, id: string) {
  state.draggedNoteId = id;
}

function groupNotesByFolder(notes: Note[]) {
  const groupedByFolders: NotesFolder = {};
  notes.forEach((note) => {
    if (note.folder !== "") {
      const folder = note.folder.trim();
      const isFolderArrayExists = groupedByFolders[folder];
      if (isFolderArrayExists) {
        groupedByFolders[folder].push(note);
      } else {
        groupedByFolders[folder] = [];
        groupedByFolders[folder].push(note);
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
