<script lang="ts">
  import { onDestroy } from 'svelte';

  // Two-click confirm instead of window.confirm, which WKWebView doesn't reliably support.
  let {
    label,
    confirmLabel = 'Click again to confirm',
    onconfirm,
  }: { label: string; confirmLabel?: string; onconfirm: () => void } = $props();

  let armed = $state(false);
  let timer: ReturnType<typeof setTimeout> | undefined;

  function click() {
    clearTimeout(timer);
    if (armed) {
      armed = false;
      onconfirm();
      return;
    }
    armed = true;
    timer = setTimeout(() => (armed = false), 3000);
  }

  onDestroy(() => clearTimeout(timer));
</script>

<button type="button" class:danger={armed} onclick={click}>{armed ? confirmLabel : label}</button>
