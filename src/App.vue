<script setup lang="ts">
import Sidebar from './components/Sidebar.vue';
import Editor from './components/Editor.vue';
</script>

<template>
  <div class="container">
    <Sidebar />
    <RouterView />
  </div>
</template>

<style >
.container {
  display: grid;
  grid-template-columns: 0.5fr 1.5fr;
  grid-template-rows: 1fr;
  gap: 0px 1em;
  grid-auto-flow: row;
  justify-items: stretch;
  grid-template-areas:
    "Control Editor";

  height: 85vh;
  padding: 0.5em;
}

.Editor {
  grid-area: Editor;
}
</style>
