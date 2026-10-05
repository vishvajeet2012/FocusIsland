<script lang="ts">
  interface SegmentOption {
    value: string;
    label: string;
  }

  export let options: SegmentOption[];
  export let value: string;
  export let ariaLabel: string;
  export let onchange: (value: string) => void;

  $: selectedIndex = Math.max(0, options.findIndex((option) => option.value === value));
</script>

<div class="segments" role="tablist" aria-label={ariaLabel} style={`--segments:${options.length};--active:${selectedIndex}`}>
  <span class="selector" aria-hidden="true"></span>
  {#each options as option}
    <button
      type="button"
      role="tab"
      aria-selected={value === option.value}
      class:active={value === option.value}
      onclick={() => onchange(option.value)}
    >{option.label}</button>
  {/each}
</div>

<style>
  .segments {
    --gap: 2px;
    position: relative;
    display: grid;
    grid-template-columns: repeat(var(--segments), minmax(0, 1fr));
    gap: var(--gap);
    padding: 3px;
    border: 1px solid var(--surface-border);
    border-radius: 11px;
    background: var(--surface-control);
    isolation: isolate;
  }
  .selector {
    position: absolute;
    z-index: -1;
    inset-block: 3px;
    left: calc(3px + var(--active) * ((100% - 6px - (var(--segments) - 1) * var(--gap)) / var(--segments) + var(--gap)));
    width: calc((100% - 6px - (var(--segments) - 1) * var(--gap)) / var(--segments));
    border-radius: 8px;
    background: var(--surface-selected);
    box-shadow: inset 0 1px rgba(255,255,255,.04), 0 1px 5px rgba(0,0,0,.18);
    transition: left var(--tab-duration) var(--ease-standard);
  }
  button {
    min-width: 72px;
    height: 27px;
    padding: 0 10px;
    border: 0;
    border-radius: 8px;
    color: var(--shell-muted);
    background: transparent;
    font: inherit;
    font-size: 11px;
    cursor: pointer;
    transition: color var(--motion-fast) var(--ease-standard);
  }
  button:hover, button.active { color: var(--shell-text); }
  button:focus-visible { outline: 2px solid currentColor; outline-offset: -2px; }
</style>
