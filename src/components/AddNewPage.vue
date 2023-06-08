<template>
    <div>
        <button @click="handleAddNew">Add New</button>
    </div>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router';
import { store } from '../utils/store';

const router = useRouter();

function handleAddNew() {
    store.note = null;
    router.push(`/editor/`)
}
</script>
