<template>
    <div class="Editor">
        <div class="editor-header">
            <input v-model="state.title" @keyup="onKeyUp"/>
            <div class="editor-actions">
                <button aria-label="Delete" class="delete" v-if="state.editedNote?.id" @click="handleDeleteNote">
                    <Delete/>
                </button>
            </div>
        </div>

        <textarea class="text-container" v-model="state.description" autofocus spellcheck="true" @keyup="onKeyUp"/>
        <div>
            <div class="editor-actions">
                <input v-model="state.tag" @keyup.enter="handleAddNewTag"/>
                <button aria-label="Add new tag" @click="handleAddNewTag">
                    <Tag/>
                </button>
            </div>
            <div class="tags-container">
                <div v-for="tag in state.tags" class="tags-inner-container">
                    <span @click="handleTagClick(tag)">#{{ tag }}</span>
                    <button class="delete-tag delete mini" @click="handleDeleteTag(tag)">
                        <Delete/>
                    </button>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import Delete from './icons/Delete.vue';
import Tag from './icons/Tag.vue';
import {invoke} from "@tauri-apps/api/tauri";
import {reactive, watch} from 'vue';
import {onBeforeRouteUpdate, useRoute, useRouter} from 'vue-router'
import {store, Store} from '../utils/store';
import {Note} from "../utils/utils";

interface State {
    editedNote: Note | null,
    title: string,
    description: string,
    tag: string,
    tags: string[],
    typingTimer: any
}

const state: State = reactive({
    editedNote: null,
    title: "",
    description: "",
    tag: '',
    tags: [],
    folderId: '',
    typingTimer: null,
})

const route = useRoute();
const router = useRouter();

function stateCleanup() {
    state.editedNote = null;
    state.title = '';
    state.description = "";
    state.tag = "";
    state.tags = [];
}

function handleAddNewTag() {
    state.tags.push(state.tag);
    state.tag = "";
    handleSave();
}

function handleDeleteTag(tag: string) {
    const filtered = state.tags.filter(t => t !== tag);
    state.tags = filtered;
    handleSave();
}

function handleTagClick(tag: string) {
    const isAlreadyAdded = store.filteredTags.includes(tag);
    if (isAlreadyAdded) {
        return;
    }

    store.filteredTags.push(tag);
}

async function handleDeleteNote() {
    if (state.editedNote) {
        await invoke("delete_note", {
            id: state.editedNote.id
        });
        store.lastUpdate = Date.now();
        store.note = null;
        stateCleanup();
        router.push(`/editor`);
    }
}

async function handleSave() {
    if (!state.title && !state.description) {
        return;
    }

    const id = Date.now().toString();
    const note = {
        id: store.note ? store.note.id : id,
        title: state.title,
        description: state.description,
        tags: state.tags,
        folderId: '',
    }
    await invoke("save_note", {
        ...note,
    })

    store.note = note;
    state.editedNote = note;
    store.lastUpdate = Date.now();
}

function findAndSetNote(newId: string) {
    const note = store.notes.find(note => note.id === newId);
    if (note) {
        state.editedNote = note;
        state.description = note.description;
        state.title = note.title;
        state.tags = note.tags;
    }
}

function setDefaultNote() {
    state.editedNote = null;
    state.description = ""
    state.title = ""
    state.tag = ""
    state.tags = []
}

function onKeyUp() {
    clearTimeout(state.typingTimer);
    state.typingTimer = setTimeout(handleSave, 500);
}

watch(
    () => route.params.id,
    async (newId) => {
        if (newId) {
            findAndSetNote(newId as string);
        } else {
            setDefaultNote();
        }
    },
    {
        immediate: true
    }
)

onBeforeRouteUpdate(() => {
    clearTimeout(state.typingTimer);
})

</script>

<style scoped>
.text-container {
    width: 100%;
    height: 85%;
    border: none;
    resize: none;
    margin-bottom: var(--big-gap);
}

.tags-container {
    display: flex;
    gap: 0.5em;
}


.Editor {
    grid-area: Editor;
    padding: var(--padding-inner);
}

.editor-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: var(--big-gap);
}

.editor-actions {
    display: flex;
    gap: 1rem;
}

.tags-container span {
    cursor: pointer;
    opacity: 0.6;
}

.delete-tag {
    display: none;
}

.tags-inner-container {
    display: flex;
    align-items: center;
}

.tags-inner-container:hover > .delete-tag {
    display: block;
    margin-left: 0.2rem;
}
</style>
