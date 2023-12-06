<template>
    <div>
        <FilteredTags @handle-clear-filtered-tag="handleClearFilteredTag" />
        <ul>
            <li @dragstart="handleDragStart" draggable="true" v-for="note in excludeNotesBasedOnAppliedFilter()"
                :id="note.id" :key="note.id" :class="{ 'active-note': store.note?.id === note.id }"
                @click="handleNoteClick(note)">
                <a>
                    {{ note.title }}
                </a>
            </li>
        </ul>
    </div>
</template>

<script lang="ts" setup>
import FilteredTags from "./FilteredTags.vue";
import { useRouter } from "vue-router";
import { invoke } from "@tauri-apps/api/tauri";
import { onBeforeMount, watch } from "vue";
import { Note, NoteStorage } from "../utils/interfaces";
import { store } from "../utils/store";

function handleDragStart(e: DragEvent) {
    const target = e.target as HTMLElement;
    store.dragTarget = target.id;
}

const router = useRouter();

function parseAndSaveNotesToStore(data: NoteStorage) {
    const parsed = Object.values(data).map((v) => v);
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

function handleClearFilteredTag(tag: string) {
    const filtered = store.filteredTags.filter((t) => t !== tag);
    store.filteredTags = filtered;
}

function excludeNotesBasedOnAppliedFilter() {
    if (store.filteredTags.length === 0) {
        return store.notes;
    }
    const filtered = store.notes.filter((n) => {
        const t = n.tags.some((el) => store.filteredTags.includes(el));
        return t;
    });

    return filtered;
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
                excludeNotesBasedOnAppliedFilter();
            }
        }
    }
);

watch(
    () => store.filteredTags,
    (newFilteredTags) => {
        excludeNotesBasedOnAppliedFilter();
    },
    {
        deep: true,
    }
);
</script>

<style scoped>
li {
    cursor: pointer;
    padding: 0.3rem;
    opacity: 0.6;
    display: -webkit-box;
    overflow: hidden;
    -webkit-box-orient: vertical;
    -webkit-line-clamp: 2;
    text-overflow: ellipsis;
    word-break: break-word;
    word-wrap: break-word;
}

li:hover {
    background-color: var(--dark-background-accent);
    border-radius: var(--border-radius);

}

.active-note {
    opacity: 1;
    background-color: var(--dark-background-accent);
    border-radius: var(--border-radius);
}

ul {
    max-height: 85vh;
    overflow: scroll;
    list-style-type: none;
    margin: 0;
    padding: 0;
    -ms-overflow-style: none;
    /* Internet Explorer 10+ */
    scrollbar-width: none;
}

ul::-webkit-scrollbar {
    display: none;
}

.drop {
    height: 100px;
    border: 1px solid salmon;
}
</style>
