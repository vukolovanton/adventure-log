export interface Note {
    id: string,
    title: string,
    description: string,
    tags: string[]
}

export interface NoteStorage {
    [key: string]: Note;
}