<script lang="ts">
  import Icon from "./Icon.svelte";
  import type { ComponentProps } from "svelte";

  export let icon: ComponentProps<Icon>["name"];
  export let label: string;
  export let size = 30;
  export let disabled = false;
  export let danger = false;
  export let onclick: (event: MouseEvent) => void = () => undefined;
</script>

<button
  class:danger
  type="button"
  aria-label={label}
  title={label}
  {disabled}
  style:width={`${size}px`}
  style:height={`${size}px`}
  {onclick}
>
  <Icon name={icon} size={15} />
</button>

<style>
  button {
    display: inline-grid;
    flex: 0 0 auto;
    place-items: center;
    padding: 0;
    border: 0;
    border-radius: 999px;
    color: inherit;
    background: color-mix(in srgb, currentColor 8%, transparent);
    cursor: pointer;
    transition: filter var(--motion-fast) var(--ease-standard), transform var(--motion-fast) var(--ease-standard), background var(--motion-fast) var(--ease-standard);
  }
  button:hover:not(:disabled) { filter: brightness(1.08); background: color-mix(in srgb, currentColor 13%, transparent); }
  button:active:not(:disabled) { transform: scale(.97); }
  button:focus-visible { outline: 2px solid currentColor; outline-offset: 2px; }
  button:disabled { opacity: .38; cursor: default; }
  button.danger:hover { color: #8d1f24; background: rgba(180, 34, 42, .12); }
</style>
