<template>
    <div class="tags-inner-container">
        <span :class="large ? 'tag-large' : 'tag'" @click="$emit('handleTagClick', tag)">#{{ tag }}</span>
        <slot></slot>
    </div>
</template>

<script setup lang="ts">
defineProps<{
    tag: string;
    large?: Boolean;
}>();
defineEmits<{
    (e: "handleTagClick", tag: string): void;
}>();
</script>

<style>
.tags-inner-container {
    display: flex;
    align-items: center;
    padding: 0 0.5rem;
    background-color: var(--yellow-color);
    border-radius: var(--border-radius);
}

.tags-inner-container:hover>.delete-tag {
    display: flex;
    margin-left: 0.4rem;
}

.tag {
    cursor: pointer;
    font-size: var(--font-size-small);
}

.tag-large {
    cursor: pointer;
    padding: 0.5rem;
    font-size: var(--font-size-large);
}
</style>
