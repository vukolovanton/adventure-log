export interface Note {
    id: string,
    title: string,
    description: string,
    tags: string[],
    folder_id: string,
}

export interface NoteStorage {
    [key: string]: Note;
}