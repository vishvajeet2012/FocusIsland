<script lang="ts">
  import { createEventDispatcher, onDestroy } from "svelte";
  import type { Task } from "../../types/task";
  import Icon from "../common/Icon.svelte";
  import IconButton from "../common/IconButton.svelte";
  import { lockCollapse, unlockCollapse } from "../../stores/app";

  export let task: Task;
  export let hour12 = true;
  export let dragging = false;

  const dispatch = createEventDispatcher<{
    toggle: { task: Task; completed: boolean };
    edit: Task;
    delete: Task;
    duplicate: Task;
    reminder: Task;
    focus: Task;
    dragstart: Task;
    dragover: Task;
    dragend: Task;
  }>();

  let menuOpen = false;
  let menuRoot: HTMLDivElement;

  function toggleMenu(event: MouseEvent): void {
    event.stopPropagation();
    menuOpen = !menuOpen;
    menuOpen ? lockCollapse() : unlockCollapse();
  }

  function closeMenu(): void {
    if (!menuOpen) return;
    menuOpen = false;
    unlockCollapse();
  }

  function run(action: "edit" | "delete" | "duplicate" | "reminder" | "focus"): void {
    closeMenu();
    dispatch(action, task);
  }

  function outside(event: PointerEvent): void {
    if (menuOpen && menuRoot && !menuRoot.contains(event.target as Node)) closeMenu();
  }

  function keydown(event: KeyboardEvent): void {
    if (event.key === "Escape") closeMenu();
  }

  onDestroy(() => { if (menuOpen) unlockCollapse(); });

  $: timeLabel = new Intl.DateTimeFormat(undefined, { hour: "numeric", minute: "2-digit", hour12 }).format(task.updatedAt);
</script>

<svelte:window onpointerdown={outside} onkeydown={keydown} />
<div
  class="task-row"
  role="listitem"
  class:completed={task.completed}
  class:dragging
  draggable={!task.completed}
  ondragstart={(event) => { event.dataTransfer?.setData("text/plain", String(task.id)); dispatch("dragstart", task); }}
  ondragover={(event) => { event.preventDefault(); dispatch("dragover", task); }}
  ondragend={() => dispatch("dragend", task)}
>
  <button class="checkbox" class:checked={task.completed} type="button" aria-label={task.completed ? `Mark ${task.title} incomplete` : `Complete ${task.title}`} onclick={() => dispatch("toggle", { task, completed: !task.completed })}>
    <Icon name="check" size={11} strokeWidth={2.3} />
  </button>
  <button class="task-main" type="button" ondblclick={() => dispatch("edit", task)}>
    <span class="title">{task.title}</span>
    <span class="time">{task.dueAt ? new Intl.DateTimeFormat(undefined, { hour: "numeric", minute: "2-digit", hour12 }).format(task.dueAt) : timeLabel}</span>
  </button>
  <div class="row-actions">
    {#if !task.completed}<IconButton icon="play" label={`Focus on ${task.title}`} size={25} onclick={() => dispatch("focus", task)} />{/if}
    <IconButton icon="edit" label={`Edit ${task.title}`} size={25} onclick={() => dispatch("edit", task)} />
    <div class="menu-root" bind:this={menuRoot}>
      <IconButton icon="more" label={`More options for ${task.title}`} size={25} onclick={toggleMenu} />
      {#if menuOpen}
        <div class="context-menu" role="menu">
          <button type="button" role="menuitem" onclick={() => run("edit")}><Icon name="edit" size={13} />Edit</button>
          <button type="button" role="menuitem" onclick={() => run("reminder")}><Icon name="bell" size={13} />Set reminder</button>
          <button type="button" role="menuitem" onclick={() => run("focus")}><Icon name="play" size={13} />Start focus</button>
          <button type="button" role="menuitem" onclick={() => run("duplicate")}><Icon name="copy" size={13} />Duplicate</button>
          <span class="divider"></span>
          <button class="danger" type="button" role="menuitem" onclick={() => run("delete")}><Icon name="trash" size={13} />Delete</button>
        </div>
      {/if}
    </div>
  </div>
</div>

<style>
  .task-row { position: relative; display: flex; align-items: center; gap: 7px; min-height: 36px; padding: 3px 4px; border-radius: 9px; transition: opacity var(--motion-normal) ease, background var(--motion-fast) ease; }
  .task-row:hover { background: rgba(255,255,255,.22); }
  .task-row.dragging { opacity: .45; }
  .checkbox { display: grid; flex: 0 0 auto; place-items: center; width: 17px; height: 17px; padding: 0; border: 1px solid rgba(23,34,26,.4); border-radius: 50%; color: transparent; background: rgba(255,255,255,.18); cursor: pointer; transition: color var(--motion-normal) var(--ease-out), background var(--motion-normal) var(--ease-out), transform var(--motion-fast) ease; }
  .checkbox.checked { color: white; border-color: #35573d; background: #35573d; }
  .checkbox:active { transform: scale(.88); }
  .checkbox:focus-visible { outline: 2px solid #263c2b; outline-offset: 2px; }
  .task-main { display: grid; min-width: 0; flex: 1; gap: 1px; padding: 0; border: 0; text-align: left; color: inherit; background: transparent; font: inherit; cursor: default; }
  .title { overflow: hidden; color: #172019; font-size: 11px; font-weight: 620; line-height: 1.2; text-overflow: ellipsis; white-space: nowrap; transition: opacity var(--motion-normal) ease; }
  .time { color: rgba(23,32,25,.49); font-size: 9px; }
  .completed .title { opacity: .55; text-decoration: line-through; text-decoration-thickness: 1px; }
  .row-actions { display: flex; align-items: center; opacity: 0; transform: translateX(2px); transition: opacity var(--motion-fast) ease, transform var(--motion-fast) ease; }
  .task-row:hover .row-actions, .task-row:focus-within .row-actions { opacity: 1; transform: none; }
  .menu-root { position: relative; }
  .context-menu { position: absolute; z-index: 30; top: 28px; right: 0; display: grid; width: 134px; padding: 5px; border: 1px solid rgba(255,255,255,.09); border-radius: 10px; color: #f1f3f2; background: #171a18; box-shadow: 0 12px 32px rgba(0,0,0,.38); animation: pop var(--motion-fast) var(--ease-out) both; }
  .context-menu button { display: flex; align-items: center; gap: 8px; width: 100%; padding: 7px 8px; border: 0; border-radius: 6px; color: inherit; text-align: left; background: transparent; font: inherit; font-size: 10px; cursor: pointer; }
  .context-menu button:hover { background: rgba(255,255,255,.08); }
  .context-menu button.danger { color: #f2aaa7; }
  .divider { height: 1px; margin: 3px 5px; background: rgba(255,255,255,.08); }
  @keyframes pop { from { opacity: 0; transform: translateY(-3px) scale(.98); } }
</style>
