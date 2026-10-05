<script lang="ts">
  import { remindersStore, addReminder, removeReminder } from "../../stores/reminders";
  import { settingsStore } from "../../stores/settings";
  import { showToast } from "../../stores/toasts";
  import { formatReminderTime } from "../../lib/time";
  import Icon from "../common/Icon.svelte";
  import IconButton from "../common/IconButton.svelte";
  import ReminderModal from "./ReminderModal.svelte";

  let modalOpen = false;
  $: upcoming = $remindersStore.filter((reminder) => !reminder.completed && reminder.remindAt >= Date.now());

  async function save(event: CustomEvent<{ title: string; remindAt: number; taskId: number | null }>): Promise<void> {
    try {
      await addReminder(event.detail.title, event.detail.remindAt, event.detail.taskId);
      modalOpen = false;
      showToast("Reminder scheduled", "success");
    } catch (error) { showToast(error instanceof Error ? error.message : "Couldn't schedule reminder", "warning"); }
  }

  async function remove(id: number): Promise<void> {
    try { await removeReminder(id); showToast("Reminder removed"); }
    catch (error) { showToast(error instanceof Error ? error.message : "Couldn't remove reminder", "warning"); }
  }
</script>

<section class="card events-card" aria-labelledby="events-heading">
  <header>
    <div><h2 id="events-heading">Events</h2><span>Local reminders</span></div>
    <button type="button" aria-label="New reminder" title="New reminder" onclick={() => modalOpen = true}><Icon name="plus" size={13} /></button>
  </header>
  <div class="section-label"><span>Upcoming</span><b>{upcoming.length}</b></div>
  <div class="event-list" class:empty={upcoming.length === 0}>
    {#if upcoming.length === 0}
      <div class="empty-state">
        <span><Icon name="calendar" size={17} /></span>
        <p>No reminders for today.</p>
        <button type="button" onclick={() => modalOpen = true}>Add reminder</button>
      </div>
    {:else}
      {#each upcoming as reminder (reminder.id)}
        <article>
          <span class="timeline-dot"></span>
          <div><h3>{reminder.title}</h3><p>{formatReminderTime(reminder.remindAt, $settingsStore.timeFormat === "12")}</p></div>
          <IconButton icon="x" label={`Delete reminder ${reminder.title}`} size={24} onclick={() => void remove(reminder.id)} />
        </article>
      {/each}
    {/if}
  </div>
  <footer><i></i><span>Local reminders active</span><Icon name="bell" size={11} /></footer>
</section>

{#if modalOpen}<ReminderModal on:close={() => modalOpen = false} on:save={save} />{/if}

<style>
  .events-card { color: #18242b; background: var(--events-card); }
  header { display: flex; align-items: flex-start; justify-content: space-between; }
  header > div { display: grid; gap: 2px; }
  h2 { margin: 0; font-size: 13px; }
  header span { color: rgba(24,36,43,.48); font-size: 9px; }
  header button { display: grid; place-items: center; width: 26px; height: 26px; padding: 0; border: 0; border-radius: 8px; color: #263b47; background: rgba(255,255,255,.3); cursor: pointer; }
  .section-label { display: flex; align-items: center; gap: 5px; margin-top: 10px; color: rgba(24,36,43,.48); font-size: 8px; font-weight: 700; letter-spacing: .55px; text-transform: uppercase; }
  .section-label b { display: grid; place-items: center; min-width: 14px; height: 14px; border-radius: 5px; background: rgba(28,46,56,.08); font-size: 7px; }
  .event-list { min-height: 0; flex: 1; margin: 6px -3px; overflow: auto; scrollbar-width: thin; scrollbar-color: rgba(28,46,56,.16) transparent; }
  .event-list.empty { display: grid; place-items: center; }
  article { display: flex; align-items: center; gap: 7px; min-height: 39px; padding: 4px; border-radius: 9px; }
  article:hover { background: rgba(255,255,255,.2); }
  article > div { min-width: 0; flex: 1; }
  article h3 { overflow: hidden; margin: 0; font-size: 10px; line-height: 1.25; text-overflow: ellipsis; white-space: nowrap; }
  article p { margin: 2px 0 0; color: rgba(24,36,43,.49); font-size: 8.5px; }
  article :global(button) { opacity: 0; }
  article:hover :global(button), article:focus-within :global(button) { opacity: 1; }
  .timeline-dot { flex: 0 0 auto; width: 6px; height: 6px; border: 2px solid rgba(46,75,91,.28); border-radius: 50%; background: rgba(255,255,255,.36); }
  .empty-state { display: grid; justify-items: center; text-align: center; }
  .empty-state > span { display: grid; place-items: center; width: 31px; height: 31px; border-radius: 11px; color: rgba(34,59,73,.53); background: rgba(255,255,255,.25); }
  .empty-state p { margin: 7px 0 5px; color: rgba(24,36,43,.57); font-size: 9px; }
  .empty-state button { padding: 4px 7px; border: 0; border-radius: 6px; color: #263b47; background: rgba(255,255,255,.34); font: inherit; font-size: 8px; cursor: pointer; }
  footer { display: flex; align-items: center; gap: 5px; color: rgba(24,36,43,.47); font-size: 8px; }
  footer i { width: 5px; height: 5px; border-radius: 50%; background: #527d67; box-shadow: 0 0 0 3px rgba(82,125,103,.1); }
  footer :global(svg) { margin-left: auto; }
</style>
