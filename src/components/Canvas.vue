<template>
    <div @dragover.prevent id="droptarget" class="dropzone">
        <button @click="scrollIntoView">Center</button>
        <button @click="loadCanvas">Load</button>
        <vue-infinite-viewer ref="viewer" class="viewer">
            <div ref="area" class="area">
                <div v-for="note in state.notes" @mousedown="onMouseDown" class="block" :id="note.id" :key="note.id">
                    <h4 class="title">{{ note.title }}</h4>
                    <p class="content">{{ note.description }}</p>
                </div>
            </div>
        </vue-infinite-viewer>
    </div>
</template>

<script setup lang="ts">
import {ref, reactive, onMounted} from 'vue';
import {VueInfiniteViewer} from "vue3-infinite-viewer";
import {store} from "../utils/store";
import {Note} from '../utils/interfaces';
import {invoke} from "@tauri-apps/api/tauri";

interface IState {
    target: HTMLElement | null;
    notes: Note[];
    activeNote: Note | null;
}

const pos1 = ref(0);
const pos2 = ref(0);
const viewer = ref(null);
const area = ref(null);
const state: IState = reactive({
    target: null,
    notes: [],
    activeNote: null,
})

function scrollIntoView() {
    if (viewer && viewer.value) {
        (viewer.value as any).scrollTo(0, 0);
    }
}

function loadCanvas() {
}

function onMouseDown(e: MouseEvent) {
    e.preventDefault();
    pos1.value = e.clientX;
    pos2.value = e.clientY;
    state.target = (e.target as HTMLElement).offsetParent as HTMLElement;
    document.addEventListener('mousemove', elementDrag);
    document.addEventListener('mouseup', closeDragElement);
}

function elementDrag(e: MouseEvent) {
    e.preventDefault();
    const dx = e.clientX - pos1.value;
    const dy = e.clientY - pos2.value;
    pos1.value = e.clientX;
    pos2.value = e.clientY;
    if (state.target) {
        state.target.style.top = `${state.target.offsetTop + dy}px`;
        state.target.style.left = `${state.target.offsetLeft + dx}px`;
    }
}

async function updateNoteCanvasData(element: Element) {
    const note = state.notes.find(n => n.id === element.id);
    const top = window.getComputedStyle(element).top;
    const left = window.getComputedStyle(element).left;
    if (note) {
        note.canvas = {
            top: top,
            left: left,
        }
        state.activeNote = note;
    }
    await invoke("save_note", {
        ...note,
    });
}

function closeDragElement(e: MouseEvent) {
    const element = (e.target as HTMLElement)?.offsetParent;
    if (element) {
        updateNoteCanvasData(element);
    }
    state.target = null;
    document.removeEventListener('mousemove', elementDrag);
    document.removeEventListener('mouseup', closeDragElement);
}

onMounted(() => {
    // TODO: Clear event listeners
    const target = document.getElementById("droptarget");

    target!.addEventListener("drop", (event) => {
        event.preventDefault();
        const note = store.notes.find(note => note.id === store.dragTarget);
        if (note) {
            state.notes.push(note);
        }
    });
})
</script>

<style scoped>
.area {
    position: relative;
}

.block {
    width: 300px;
    max-height: 500px;
    position: absolute;
    cursor: grab;
}

.viewer {
    width: 100%;
    height: 100%;
}

.title {
    background-color: orange;
}

.content {
    background-color: aliceblue;
}
</style>
