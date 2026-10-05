<script lang="ts">
  export let name:
    | "plus" | "play" | "pause" | "check" | "x" | "more" | "edit" | "bell"
    | "copy" | "trash" | "clock" | "stopwatch" | "spark" | "settings" | "archive"
    | "chevron" | "calendar" | "grip" | "volume" | "volume-off" | "tray" = "plus";
  export let size = 16;
  export let strokeWidth = 1.8;
</script>

<svg
  width={size}
  height={size}
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width={strokeWidth}
  stroke-linecap="round"
  stroke-linejoin="round"
  aria-hidden="true"
>
  {#if name === "plus"}<path d="M12 5v14M5 12h14" />
  {:else if name === "play"}<path d="m8 5 11 7-11 7Z" />
  {:else if name === "pause"}<path d="M9 5v14M15 5v14" />
  {:else if name === "check"}<path d="m5 12 4 4L19 6" />
  {:else if name === "x"}<path d="m6 6 12 12M18 6 6 18" />
  {:else if name === "more"}<circle cx="5" cy="12" r="1" fill="currentColor" stroke="none" /><circle cx="12" cy="12" r="1" fill="currentColor" stroke="none" /><circle cx="19" cy="12" r="1" fill="currentColor" stroke="none" />
  {:else if name === "edit"}<path d="M12 20h9M16.5 3.5a2.1 2.1 0 0 1 3 3L8 18l-4 1 1-4Z" />
  {:else if name === "bell"}<path d="M18 8a6 6 0 0 0-12 0c0 7-3 7-3 9h18c0-2-3-2-3-9M10 21h4" />
  {:else if name === "copy"}<rect x="8" y="8" width="12" height="12" rx="2" /><path d="M16 8V6a2 2 0 0 0-2-2H6a2 2 0 0 0-2 2v8a2 2 0 0 0 2 2h2" />
  {:else if name === "trash"}<path d="M4 7h16M10 11v6M14 11v6M6 7l1 14h10l1-14M9 7V4h6v3" />
  {:else if name === "clock"}<circle cx="12" cy="12" r="9" /><path d="M12 7v5l3 2" />
  {:else if name === "stopwatch"}<circle cx="12" cy="13" r="8" /><path d="M12 9v4l3 2M9 2h6M12 2v3M18 6l2-2" />
  {:else if name === "spark"}<path d="m12 3 1.4 4.6L18 9l-4.6 1.4L12 15l-1.4-4.6L6 9l4.6-1.4ZM19 15l.7 2.3L22 18l-2.3.7L19 21l-.7-2.3L16 18l2.3-.7Z" />
  {:else if name === "settings"}<circle cx="12" cy="12" r="3" /><path d="M19.4 15a1.7 1.7 0 0 0 .3 1.9l.1.1-2.8 2.8-.1-.1a1.7 1.7 0 0 0-1.9-.3 1.7 1.7 0 0 0-1 1.6v.2h-4V21a1.7 1.7 0 0 0-1-1.6 1.7 1.7 0 0 0-1.9.3l-.1.1L4.2 17l.1-.1a1.7 1.7 0 0 0 .3-1.9A1.7 1.7 0 0 0 3 14H2.8v-4H3a1.7 1.7 0 0 0 1.6-1 1.7 1.7 0 0 0-.3-1.9L4.2 7 7 4.2l.1.1A1.7 1.7 0 0 0 9 4.6a1.7 1.7 0 0 0 1-1.6v-.2h4V3a1.7 1.7 0 0 0 1 1.6 1.7 1.7 0 0 0 1.9-.3l.1-.1L19.8 7l-.1.1a1.7 1.7 0 0 0-.3 1.9 1.7 1.7 0 0 0 1.6 1h.2v4H21a1.7 1.7 0 0 0-1.6 1Z" />
  {:else if name === "archive"}<path d="M4 7h16v13H4ZM3 4h18v3H3ZM9 11h6" />
  {:else if name === "chevron"}<path d="m9 6 6 6-6 6" />
  {:else if name === "calendar"}<rect x="3" y="5" width="18" height="16" rx="2" /><path d="M16 3v4M8 3v4M3 10h18" />
  {:else if name === "grip"}<circle cx="9" cy="6" r="1" fill="currentColor" stroke="none" /><circle cx="15" cy="6" r="1" fill="currentColor" stroke="none" /><circle cx="9" cy="12" r="1" fill="currentColor" stroke="none" /><circle cx="15" cy="12" r="1" fill="currentColor" stroke="none" /><circle cx="9" cy="18" r="1" fill="currentColor" stroke="none" /><circle cx="15" cy="18" r="1" fill="currentColor" stroke="none" />
  {:else if name === "volume"}<path d="M11 5 6 9H2v6h4l5 4ZM15 9a4 4 0 0 1 0 6M18 6a8 8 0 0 1 0 12" />
  {:else if name === "volume-off"}<path d="M11 5 6 9H2v6h4l5 4ZM16 10l5 5M21 10l-5 5" />
  {:else if name === "tray"}<path d="M4 4h16v13H4ZM8 21h8M12 17v4" />
  {/if}
</svg>
