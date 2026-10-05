<script lang="ts">
  import { BRAND_ICON_PATH } from "../../lib/constants";

  export let size = 22;
</script>

<img src={BRAND_ICON_PATH} alt="" aria-hidden="true" width={size} height={size} draggable="false" />

<style>
  img { display: block; flex: 0 0 auto; object-fit: contain; user-select: none; }
</style>
