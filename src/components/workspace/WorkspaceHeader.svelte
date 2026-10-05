<script lang="ts">
  import { PRODUCT_NAME } from "../../lib/constants";
  import type { WorkspaceTab } from "../../types/app";
  import BrandMark from "../common/BrandMark.svelte";
  import IconButton from "../common/IconButton.svelte";
  import SegmentedControl from "../common/SegmentedControl.svelte";

  export let activeTab: WorkspaceTab;
  export let onTabChange: (tab: WorkspaceTab) => void;
  export let onCollapse: () => void;
  export let movable = false;

  const tabs = [
    { value: "workspace", label: "Workspace" },
    { value: "insights", label: "Insights" },
    { value: "customize", label: "Customize" },
  ];
</script>

<header class="workspace-header" data-tauri-drag-region={movable ? "" : undefined}>
  <div class="brand" data-tauri-drag-region={movable ? "" : undefined}>
    <span class="mark"><BrandMark size={20} /></span>
    <span>{PRODUCT_NAME}</span>
  </div>
  <div class="right">
    <SegmentedControl options={tabs} value={activeTab} ariaLabel="Workspace section" onchange={(value) => onTabChange(value as WorkspaceTab)} />
    <IconButton icon="x" label="Collapse workspace" size={31} onclick={onCollapse} />
  </div>
</header>

<style>
  .workspace-header { display: flex; align-items: center; justify-content: space-between; height: 36px; padding: 0 2px 8px; color: var(--shell-text); user-select: none; }
  .brand { display: flex; align-items: center; gap: 8px; font-size: 12px; font-weight: 670; letter-spacing: -.1px; }
  .mark { display: grid; place-items: center; width: 25px; height: 25px; border-radius: 9px; filter: drop-shadow(0 2px 5px rgba(0,0,0,.24)); }
  .right { display: flex; align-items: center; gap: 8px; }
</style>
