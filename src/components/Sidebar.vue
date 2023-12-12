<template>
    <div class="Sidebar">
        <AddNewPage />
        <NotesList />
    </div>
</template>

<script setup lang="ts">
import AddNewPage from './AddNewPage.vue';
import NotesList from './NotesList.vue';
</script>

<style>
.Sidebar {
    border-radius: var(--border-radius);
    grid-area: Sidebar;
    background-color: var(--alt-background-color);
    height: 100vh;
    padding: var(--padding-inner);
}
</style>
