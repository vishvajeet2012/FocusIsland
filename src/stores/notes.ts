import { writable } from "svelte/store";
import type { DailyNote } from "../types/app";

export interface NoteState {
  note: DailyNote;
  saving: boolean;
  saved: boolean;
}

export const notesStore = writable<NoteState>({
  note: { id: 0, date: "", content: "", createdAt: 0, updatedAt: 0 },
  saving: false,
  saved: true,
});
