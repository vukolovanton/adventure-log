<template>
    <div @dragover.prevent @drop.prevent="handleDrop" id="droptarget" class="dropzone">
        <div class="actions-container">
            <button @click="scrollIntoView" class="target mini">
                <Target />
            </button>
            <button @click="zoomIn" class="target mini">
                <Plus />
            </button>
            <button @click="zoomOut" class="target mini">
                <Minus />
            </button>
        </div>
        <vue-infinite-viewer ref="viewer" class="viewer">
            <div ref="area" class="area">
                <div v-for="note in state.notes" @dblclick="onDoubleClick" @mousedown="onMouseDown" class="block"
                    :data-id="note.id" :key="note.id">
                    <h4 class="title inner-padding">{{ note.title }}</h4>
                    <p class="content inner-padding">{{ note.description }}</p>
                </div>
            </div>
        </vue-infinite-viewer>
    </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted, nextTick, onUpdated } from 'vue';
import { VueInfiniteViewer } from "vue3-infinite-viewer";
import { store } from "../utils/store";
import { Note, NoteStorage } from '../utils/interfaces';
import { invoke } from "@tauri-apps/api/tauri";
import Target from "./icons/Target.vue";
import Plus from "./icons/Plus.vue";
import Minus from './icons/Minus.vue';
import { useRouter } from "vue-router";

const router = useRouter();

interface IState {
    target: HTMLElement | null;
    notes: Note[];
}

const initial = {
    useMouseDrag: true,
    useWheelScroll: true,
    useAutoZoom: true,
    zoomRange: [0.1, 10],
    maxPinchWheel: 10,
}

const zoom = ref(1);
const pos1 = ref(0);
const pos2 = ref(0);
const viewer = ref(null);
const area = ref(null);
const state: IState = reactive({
    target: null,
    notes: [],
});

function scrollIntoView() {
    if (viewer && viewer.value) {
        (viewer.value as any).scrollTo(0, 0);
    }
}

function zoomIn() {
    if (viewer && viewer.value) {
        zoom.value = 1;
        (viewer.value as any).setZoom(zoom.value);
    }
}

function zoomOut() {
    if (viewer && viewer.value) {
        zoom.value = 0.5;
        (viewer.value as any).setZoom(zoom.value);
    }
}

function onDoubleClick(e: MouseEvent) {
    const target = (e.target as HTMLElement).offsetParent as HTMLElement;
    if (target.dataset.id) {
        const note = state.notes.find(note => note.id === target.dataset.id);
        if (note) {
            store.note = note;
            router.push({
                path: `/editor/${note.id}`,
            });
        }
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
    let dx = e.clientX - pos1.value;
    let dy = e.clientY - pos2.value;
    if (zoom.value === 0.5) {
        dx = dx * 2;
        dy = dy * 2;
    }
    pos1.value = e.clientX;
    pos2.value = e.clientY;
    if (state.target) {
        state.target.style.top = `${state.target.offsetTop + dy}px`;
        state.target.style.left = `${state.target.offsetLeft + dx}px`;
    }
}

async function updateNoteCanvasData(element: Element) {
    const note = state.notes.find(n => n.id === (element as HTMLElement).dataset.id);
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
    console.log(element)
    if (element) {
        updateNoteCanvasData(element);
    }
    state.target = null;
    document.removeEventListener('mousemove', elementDrag);
    document.removeEventListener('mouseup', closeDragElement);
}

function handleDrop(e: MouseEvent) {
    const note = store.notes.find(note => note.id === store.dragTarget);
    if (note) {
        const isAlreadyAdded = state.notes.findIndex(n => n.id === note.id);
        if (isAlreadyAdded === -1) {
            if (!note.canvas) {
                note.canvas = {
                    top: e.clientY.toString(),
                    left: e.clientX.toString(),
                };
            }
            state.notes.push(note);
            nextTick(() => {
                const element = document.querySelector(`[data-id="${note.id}"]`) as HTMLElement;
                element.style.top = `${note.canvas!.top}px`;
                element.style.left = `${Number(note.canvas!.left) - 300}px`;
            })
        }
    }
}

function loadInitialCanvasState() {
    nextTick(() => {
        if (area.value) {
            (area.value as HTMLElement).childNodes.forEach(node => {
                if (node.nodeType === 1) {
                    const note = state.notes.find(n => n.id === (node as HTMLElement).dataset.id);
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

.actions-container {
    position: absolute;
    z-index: 100;
    top: 2%;
    right: 1%;
    display: flex;
    flex-direction: column;
    gap: var(--small-gap);
}

.target {
    background: var(--background-color);
}

.inner-padding {
    padding: 0.5em;
}
</style>
