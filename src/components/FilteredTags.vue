<template>
    <div class="applied-filters">
        <span class="tag" v-for="filter in store.filteredTags">#{{ filter }}</span>
        <button aria-label="Clear filters" v-if="store.filteredTags.length > 0" @click="$emit('handleClearFilters')">
            <Xmark />
        </button>
    </div>
</template>

<script setup lang="ts">
defineEmits(['handleClearFilters'])
import Xmark from './icons/Xmark.vue';
import { store } from '../utils/store';
</script>

<style scoped>
.applied-filters {
    display: flex;
    gap: 0.3rem;
    flex-wrap: wrap;
    margin-bottom: var(--big-gap);
}

.tag {
    font-size: 12px;
}
</style>