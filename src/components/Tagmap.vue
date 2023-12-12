<template>
    <section class="section">
        <div class="tags-container">
            <template v-for="tag in state.tags">
                <Tag :large="true" :tag="tag" @handle-tag-click="handleTagClick" />
            </template>
        </div>
    </section>
</template>

<script setup lang="ts">
interface IState {
    tags: string[]
}
import { onMounted, reactive } from 'vue'
import Tag from './Tag.vue'
import { invoke } from "@tauri-apps/api/tauri";
import { store } from "../utils/store";

const state: IState = reactive({
    tags: [],
})

function handleTagClick(tag: string) {
    const isAlreadyAdded = store.filteredTags.includes(tag);
    if (isAlreadyAdded) {
        return;
    }

    store.filteredTags.push(tag);
}

async function onLoad() {
    const tags: string[] = await invoke("get_all_tags");
    state.tags = tags;
}

onMounted(() => {
    onLoad();
})
</script>

<style scoped>
.section {
    border-radius: var(--border-radius);
    background-color: #e5e5f7;
    opacity: 0.8;
    background-image: radial-gradient(#603fe3 0.75px, transparent 0.75px), radial-gradient(#603fe3 0.75px, #e5e5f7 0.75px);
    background-size: 30px 30px;
    background-position: 0 0, 15px 15px;
}

.tags-container {
    display: flex;
    gap: 0.5em;
    margin-top: var(--small-gap);
    padding: 1rem;
    flex-wrap: wrap;

    & .tags-inner-container {
        padding: 0;
    }

    &>.tag {
        font-size: 200px;
    }
}

.tags-inner-container .tag {
    color: red;
    font-size: var(--font-size-large);
}
</style>
