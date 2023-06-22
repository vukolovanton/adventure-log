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
defineProps<{
  tag: string;
}>();
defineEmits<{
  (e: "handleTagClick", tag: string): void;
  (e: "handleDeleteTag", tag: string): void;
}>();
</script>

<style scoped></style>
