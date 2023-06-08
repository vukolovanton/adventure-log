<template>
    <div>
        <button @click="handleAddNew">Add New</button>
    </div>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";

async function greet() {
    greetMsg.value = await invoke("greet", { name: name.value });
}
</script>
