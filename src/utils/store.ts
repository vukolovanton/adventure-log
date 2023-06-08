import { reactive } from 'vue'
import { Note } from './utils'

export interface Store {
  lastUpdate: number,
  note: Note | null,
  notes: Note[]
}

export const store: Store = reactive({
  lastUpdate: 0,
  note: null,
  notes: []
})
