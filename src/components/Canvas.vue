<template>
    <div>
        <button @click="scrollIntoView">Center</button>
        <vue-infinite-viewer :options="viewerOptions" ref="viewer" class="viewer">
            <div ref="area" class="area">
                <div @mousedown="onMouseDown" class="block" id="123">
                    <h4 class="title">Title</h4>
                    <p class="content">
                        since Vue's reactivity tracking works over property access, we must always keep the same reference
                        to the reactive object. This means we can't easily "replace" a reactive object because the
                        reactivityconnection to the first reference is lost
                    </p>
                </div>
            </div>
        </vue-infinite-viewer>
    </div>
</template>

<script setup lang="ts">
import { VueInfiniteViewer } from "vue3-infinite-viewer";
import { ref, reactive } from 'vue';
interface IState {
    target: HTMLElement | null;
}
const viewerOptions = {
    zoomable: false,
}
const pos1 = ref(0);
const pos2 = ref(0);
const viewer = ref(null);
const state: IState = reactive({
    target: null,
})

function scrollIntoView() {
    viewer.value.scrollTo(0, 0);
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

function closeDragElement() {
    state.target = null;
    document.removeEventListener('mousemove', elementDrag);
    document.removeEventListener('mouseup', closeDragElement);
}
</script>

<style scoped>
.area {
    position: relative;
}

.block {
    width: 300px;
    max-height: 500px;
    position: absolute;
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
