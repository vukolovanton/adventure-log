export interface Note {
  id: string;
  title: string;
  description: string;
  tags: string[];
  folder: string;
}

export interface NoteStorage {
  [key: string]: Note;
}
