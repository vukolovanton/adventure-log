import { reactive } from 'vue'

interface Store {
  noteId: string,
}

export const store: Store = reactive({
  noteId: '',
})
