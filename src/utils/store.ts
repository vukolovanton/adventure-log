import { reactive } from "vue";
import { Note } from "./interfaces";

export interface Store {
  lastUpdate: number;
  note: Note | null;
  notes: Note[];
  filteredTags: string[];
}

export const store: Store = reactive({
  lastUpdate: 0,
  note: null,
  notes: [],
  filteredTags: [],
});
