<template>
    <details>
        <summary>New Folder 2</summary>
        <div
            id="drop-2"
            @drop.prevent="onDrop"
            @dragenter.prevent
            @dragover.prevent
            class="drop"
        ></div>
    </details>
</template>