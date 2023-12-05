export interface Note {
    id: string;
    title: string;
    description: string;
    tags: string[];
    canvas: CanvasElement | null;
}

export interface NoteStorage {
    [key: string]: Note;
}

export interface CanvasElement {
    top: string;
    left: string;
}