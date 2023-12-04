<template>
    <div ref="area" class="area">
        <div ref="elmnt" @mousedown="onMouseDown" class="block">123</div>
    </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';

const pos1 = ref(0);
const pos2 = ref(0);
const elmnt = ref(null);

function onMouseDown(e) {
    e.preventDefault();
    pos1.value = e.clientX;
    pos2.value = e.clientY;
    document.addEventListener('mousemove', elementDrag);
    document.addEventListener('mouseup', closeDragElement);
}

function elementDrag(e: MouseEvent) {
    e.preventDefault();
    const dx = e.clientX - pos1.value;
    const dy = e.clientY - pos2.value;
    pos1.value = e.clientX;
    pos2.value = e.clientY;
    if (elmnt && elmnt.value) {
        elmnt.value.style.top = `${elmnt.value.offsetTop + dy}px`;
        elmnt.value.style.left = `${elmnt.value.offsetLeft + dx}px`;
    }
}

function closeDragElement() {
    document.removeEventListener('mousemove', elementDrag);
    document.removeEventListener('mouseup', closeDragElement);
}

onMounted(() => {
});
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
