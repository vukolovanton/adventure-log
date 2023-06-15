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
  grid-area: Sidebar;
  background-color: var(--alt-background-color);
  min-height: 100vh;
  padding: var(--padding-inner);
}
</style>
