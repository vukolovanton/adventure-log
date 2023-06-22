<template>
  <div class="applied-filters">
    <template v-for="filter in store.filteredTags">
      <Tag :tag="filter" @handle-tag-click="temp" />
    </template>
  </div>
</template>

<script setup lang="ts">
import { store } from "../utils/store";
import Tag from "./Tag.vue";

const emit = defineEmits<{
  (e: "handleClearFilteredTag", tag: string): void;
}>();

function temp(tag: string) {
  emit("handleClearFilteredTag", tag);
}
</script>

<style scoped>
.applied-filters {
  display: flex;
  gap: 0.3rem;
  flex-wrap: wrap;
  margin-bottom: var(--big-gap);
}
</style>
