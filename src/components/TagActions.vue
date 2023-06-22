<template>
  <button
    tabindex="-1"
    class="delete-tag delete mini"
    @click="$emit('handleDeleteTag', tag)"
  >
    <Delete />
  </button>
</template>

<script setup lang="ts">
import Delete from "./icons/Delete.vue";

defineProps<{
  tag: string;
}>();
defineEmits<{
  (e: "handleDeleteTag", tag: string): void;
}>();
</script>

<style scoped>
.delete-tag {
  display: none;
}
</style>
