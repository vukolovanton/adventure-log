<template>
    <div @dragover.prevent @drop.prevent="handleDrop" id="droptarget" class="dropzone">
        <button @click="scrollIntoView" class="target mini">
            <Target />
        </button>
        <vue-infinite-viewer ref="viewer" class="viewer">
            <div ref="area" class="area">
                <div v-for="note in state.notes" @mousedown="onMouseDown" class="block" :id="note.id" :key="note.id">
                    <h4 class="title inner-padding">{{ note.title }}</h4>
                    <p class="content inner-padding">{{ note.description }}</p>
                </div>
            </div>
        </vue-infinite-viewer>
    </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted, nextTick } from 'vue';
import { VueInfiniteViewer } from "vue3-infinite-viewer";
import { store } from "../utils/store";
import { Note, NoteStorage } from '../utils/interfaces';
import { invoke } from "@tauri-apps/api/tauri";
import Target from "./icons/Target.vue";

interface IState {
    target: HTMLElement | null;
    notes: Note[];
}

const pos1 = ref(0);
const pos2 = ref(0);
const viewer = ref(null);
const area = ref(null);
const state: IState = reactive({
    target: null,
    notes: [],
})

function scrollIntoView() {
    if (viewer && viewer.value) {
        (viewer.value as any).scrollTo(0, 0);
    }
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

function handleDrop() {
    const note = store.notes.find(note => note.id === store.dragTarget);
    if (note) {
        state.notes.push(note);
    }
}

function loadInitialCanvasState() {
    nextTick(() => {
        if (area.value) {
            (area.value as HTMLElement).childNodes.forEach(node => {
                if (node.nodeType === 1) {
                    const note = state.notes.find(n => n.id === (node as HTMLElement).id);
                    if (note && note.canvas) {
                        (node as HTMLElement).style.top = note.canvas.top;
                        (node as HTMLElement).style.left = note.canvas.left;
                    }
                }
            })
        }
    });

}

onMounted(async () => {
    const data: NoteStorage = await invoke("get_all_notes_with_canvas");
    if (data) {
        Object.entries(data).map(v => {
            state.notes.push(v[1])
        });
    }
    loadInitialCanvasState()
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
    background-color: #e5e5f7;
    opacity: 0.8;
    background-image: radial-gradient(#603fe3 0.5px, #e5e5f7 0.5px);
    background-size: 10px 10px;
}

.title {
    background: var(--blue-color);
    color: var(--alt-text-color);
}

.content {
    background-color: aliceblue;
}

.target {
    position: absolute;
    z-index: 100;
    background: var(--background-color);
    top: 2%;
    right: 1%;
}

.inner-padding {
    padding: 0.5em;
}
</style>
