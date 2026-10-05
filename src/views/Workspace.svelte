<script lang="ts">
  import TaskCard from "../components/tasks/TaskCard.svelte";
  import TimerCard from "../components/timer/TimerCard.svelte";
  import NotesCard from "../components/notes/NotesCard.svelte";
  import EventsCard from "../components/events/EventsCard.svelte";

  export let timerHighlighted = false;
  let taskCard: { focusInput: () => void };
  export function focusTaskInput(): void { taskCard?.focusInput(); }
</script>

<div class="workspace-grid">
  <TaskCard bind:this={taskCard} />
  <TimerCard highlighted={timerHighlighted} />
  <NotesCard />
  <EventsCard />
</div>

<style>
  .workspace-grid { display: grid; grid-template-columns: minmax(285px, 1.72fr) minmax(145px, .82fr) minmax(180px, 1fr) minmax(180px, 1fr); gap: 8px; min-height: 0; height: 100%; }
  @media (max-width: 780px) {
    .workspace-grid { grid-template-columns: 1.35fr 1fr; grid-template-rows: 1fr 1fr; }
  }
</style>
