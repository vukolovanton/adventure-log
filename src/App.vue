<script setup lang="ts">
import Sidebar from "./components/Sidebar.vue";
</script>

<template>
    <div class="container">
        <Sidebar />
        <RouterView />
    </div>
</template>

<style>
.container {
    display: grid;
    grid-template-columns: 0.6fr 1.5fr;
    grid-template-rows: 1fr;
    gap: 0px 1rem;
    grid-auto-flow: row;
    justify-items: stretch;
    grid-template-areas: "Sidebar Editor";
}

.Editor {
    grid-area: Editor;
    max-height: 70vh;
}
</style>
