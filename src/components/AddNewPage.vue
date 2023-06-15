<template>
    <div class="controls">
        <button aria-label="Add new note" @click="handleAddNew" class="main">Add New</button>
        <button aria-label="Settings" @click="handleGoHome" class="settings">
            <Gear />
        </button>
    </div>
</template>

<script setup lang="ts">
import Gear from './icons/Gear.vue';
import { useRouter } from 'vue-router';
import { store } from '../utils/store';

const router = useRouter();

function handleAddNew() {
    store.note = null;
    router.push(`/editor`)
}

function handleGoHome() {
    router.push(`/`)
}
</script>

<style scoped>
.controls {
    display: flex;
    align-items: center;
    justify-content: space-between;
    flex-direction: row;
    margin-bottom: 0.5rem;
}
</style>