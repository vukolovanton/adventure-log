<template>
    <div ref="area" class="area">
        <div @mousedown="onMouseDown" class="block">123</div>
    </div>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue';
interface IState {
    target: HTMLElement | null;
}
const pos1 = ref(0);
const pos2 = ref(0);
const state: IState = reactive({
    target: null,
})

function onMouseDown(e: MouseEvent) {
    e.preventDefault();
    pos1.value = e.clientX;
    pos2.value = e.clientY;
    state.target = e.target as HTMLElement;
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
    background-color: orange;
    position: relative;
}

.block {
    width: 50px;
    height: 20px;
    background-color: pink;
    position: absolute;
}
</style>

<!-- const area = ref(null)
let startX = 0
let startY = 0

function handleMouseDown(e) {
    startX = e.pageX - area.value.offsetLeft
    startY = e.pageY - area.value.offsetTop
    area.value.style.cursor = 'grabbing'
    document.addEventListener('mousemove', handleMouseMove)
    document.addEventListener('mouseup', handleMouseUp)
}

function handleMouseMove(e) {
    area.value.style.left = e.pageX - startX + 'px'
    area.value.style.top = e.pageY - startY + 'px'
}

function handleMouseUp() {
    area.value.style.cursor = 'grab'
    document.removeEventListener('mousemove', handleMouseMove)
    document.removeEventListener('mouseup', handleMouseUp)
}

onMounted(() => {
    area.value.addEventListener('mousedown', handleMouseDown)
})

onUnmounted(() => {
    area.value.removeEventListener('mousedown', handleMouseDown)
}) -->
