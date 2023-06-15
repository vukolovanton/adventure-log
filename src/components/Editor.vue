<template>
  <div class="Editor">
    <div class="editor-header">
      <input v-model="state.title" />
      <div class="editor-actions">
        <button class="delete" v-if="state.editedNote?.id" @click="handleDeleteNote">
          <Delete />
        </button>
        <button class="save" @click="handleSave">
          <Check />
        </button>
      </div>
    </div>

    <textarea class="text-container" v-model="state.description" autofocus="true" spellcheck="true" />
    <div>
      <div class="editor-actions">
        <input v-model="state.tag" />
        <button @click="handleAddNewTag">
          <Tag />
        </button>
      </div>
      <div class="tags-container">
        <span v-for="tag in state.tags" @click="handleTagClick(tag)">#{{ tag }}</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import Delete from './icons/Delete.vue';
import Check from './icons/Check.vue';
import Tag from './icons/Tag.vue';
import { invoke } from "@tauri-apps/api/tauri";
import { reactive, watch, onBeforeUnmount, onMounted } from 'vue';
import { useRoute, useRouter } from 'vue-router'
import { store, Store } from '../utils/store';
import { Note } from "../utils/utils";

interface State {
  editedNote: Note | null,
  title: string,
  description: string,
  tag: string,
  tags: string[]
}

const state: State = reactive({
  editedNote: null,
  title: "",
  description: "",
  tag: '',
  tags: []
})

const route = useRoute();
const router = useRouter();

function handleAddNewTag() {
  state.tags.push(state.tag);
  state.tag = "";
  handleSave();
}

function handleTagClick(tag: string) {
  store.filteredTags.push(tag);
}

async function handleDeleteNote() {
  if (state.editedNote) {
    await invoke("delete_note", {
      id: state.editedNote.id
    });
    store.lastUpdate = Date.now();
    store.note = null;
    router.push(`/editor`)
  }
}

async function handleSave() {
  const id = Date.now().toString();
  const note = {
    id: store.note ? store.note.id : id,
    title: state.title,
    description: state.description,
    tags: state.tags,
  }
  await invoke("save_note", {
    ...note,
  })

  store.note = note;
  store.lastUpdate = Date.now();
}

function findAndSetNote(newId: string) {
  const note = store.notes.find(note => note.id === newId);
  if (note) {
    state.editedNote = note;
    state.description = note.description;
    state.title = note.title;
    state.tags = note.tags;
  }
}

function setDefaultNote() {
  state.editedNote = null;
  state.description = ""
  state.title = ""
  state.tag = ""
  state.tags = []
}

watch(
  () => route.params.id,
  async (newId) => {
    if (newId) {
      findAndSetNote(newId as string);
    } else {
      setDefaultNote();
    }
  },
  {
    immediate: true
  }
)

</script>

<style scoped>
.text-container {
  width: 100%;
  height: 85%;
  border: none;
  resize: none;
  margin-bottom: var(--big-gap);
}

.tags-container {
  display: flex;
  gap: 0.5em;
}


.Editor {
  grid-area: Editor;
  padding: var(--padding-inner);
}

.editor-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: var(--big-gap);
}

.editor-actions {
  display: flex;
  gap: 1rem;
}

.tags-container span {
  cursor: pointer;
}
</style>
