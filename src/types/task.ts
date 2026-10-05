export interface Task {
  id: number;
  title: string;
  notes: string | null;
  completed: boolean;
  createdAt: number;
  updatedAt: number;
  dueAt: number | null;
  reminderAt: number | null;
  estimatedFocusMinutes: number | null;
  sortOrder: number;
  archivedAt: number | null;
}

export interface TaskInput {
  title: string;
  notes?: string | null;
  dueAt?: number | null;
  estimatedFocusMinutes?: number | null;
}

export interface TaskUpdate extends TaskInput {
  id: number;
}
