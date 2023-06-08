<template>
  <div class="Control">
    <AddNewPage />
    <NotesList />
  </div>
</template>

<script setup lang="ts">
import AddNewPage from './AddNewPage.vue';
import NotesList from './NotesList.vue';

</script>

<style>
.Control {
  grid-area: Control;
}
</style>
