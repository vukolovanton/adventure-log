<template>
    <div>
        <FilteredTags @handle-clear-filters="handleClearFilters"/>

        <details>
            <summary>New Folder 1</summary>
            <div
                id="drop-1"
                @drop.prevent="onDrop"
                @dragenter.prevent
                @dragover.prevent
                class="drop"
            ></div>
        </details>

        <details>
            <summary>New Folder 2</summary>
            <div
                id="drop-2"
                @drop.prevent="onDrop"
                @dragenter.prevent
                @dragover.prevent
                class="drop"
            ></div>
        </details>

        <ul>
            <li
                @dragstart="(event) => onDragStart(event, note.id)"
                :id="note.id"
                draggable="true"
                v-for="note in state.notes"
                :class="{ 'active-note': store.note?.id === note.id }"
                @click="handleNoteClick(note)"
            >
                <a>
                    {{ note.title }}
                </a>
            </li>
        </ul>
    </div>
</template>

<script lang="ts" setup>
import FilteredTags from "./FilteredTags.vue";
import {useRouter} from "vue-router";
import {invoke} from "@tauri-apps/api/tauri";
import {reactive, onBeforeMount, watch} from "vue";
import {Note, NoteStorage} from "../utils/utils";
import {store} from "../utils/store";

interface IState {
    notes: Note[];
    isActive: boolean;
    draggedNoteId: string;
}

const state: IState = reactive({
    notes: [],
    isActive: false,
    draggedNoteId: "",
});
const router = useRouter();

async function onDrop(event: DragEvent) {
    console.log("ended ->> ", state.draggedNoteId);
    console.log((event.target as HTMLElement).id);
    const folderId = (event.target as HTMLElement).id;

    await invoke("update_note_folder_id", {
        noteId: state.draggedNoteId,
        folderId: folderId,
    });
}

function onDragStart(event: DragEvent, id: string) {
    state.draggedNoteId = id;
}

async function requestNotesList() {
    const data: NoteStorage = await invoke("get_all_notes");
    const parsed = Object.values(data).map((v) => v);
    state.notes.length = 0;
    state.notes.push(...parsed);
    store.notes = parsed;
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

function updateNotesBasedOnFilter(filteredTags: string[]) {
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
                updateNotesBasedOnFilter(store.filteredTags);
            }
        }
    }
);

watch(
    () => store.filteredTags,
    (newFilteredTags) => {
        updateNotesBasedOnFilter(newFilteredTags);
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
