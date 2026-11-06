<script lang="ts">
  import { errorMessage } from '$lib/api';
  import { mediaSrc, storeImage } from '$lib/media';
  import type { CardSide } from '$lib/types';

  let {
    label,
    side = $bindable(),
    autofocus = false,
  }: { label: string; side: CardSide; autofocus?: boolean } = $props();

  const id = $props.id();
  let textarea: HTMLTextAreaElement;
  let fileInput: HTMLInputElement;
  let error = $state('');

  export function focus() {
    textarea.focus();
  }

  async function addFiles(files: File[]) {
    for (const file of files) {
      try {
        side.images.push(await storeImage(file));
        error = '';
      } catch (e) {
        error = errorMessage(e);
      }
    }
  }

  function onpaste(e: ClipboardEvent) {
    // getAsFile() must run synchronously, before the clipboard data is released.
    const files = Array.from(e.clipboardData?.items ?? [])
      .filter((item) => item.kind === 'file' && item.type.startsWith('image/'))
      .map((item) => item.getAsFile())
      .filter((f): f is File => f !== null);
    if (files.length === 0) return; // plain text paste proceeds normally
    e.preventDefault();
    void addFiles(files);
  }

  async function onFilesChosen() {
    await addFiles(Array.from(fileInput.files ?? []));
    fileInput.value = '';
  }
</script>

<div class="side stack">
  <label for={id}>{label}</label>
  <textarea
    {id}
    bind:this={textarea}
    bind:value={side.text}
    {onpaste}
    rows="6"
    placeholder="Type text, or paste an image (⌘V)"
    {@attach (node) => {
      if (autofocus) node.focus();
    }}
  ></textarea>
  <div class="row">
    <button type="button" onclick={() => fileInput.click()}>Add image…</button>
    <input
      bind:this={fileInput}
      type="file"
      accept="image/*"
      multiple
      hidden
      onchange={onFilesChosen}
    />
  </div>
  {#if error}<p class="error">{error}</p>{/if}
  {#if side.images.length > 0}
    <div class="previews">
      {#each side.images as name, i (name)}
        <div class="tile">
          <img src={mediaSrc(name)} alt="" />
          <button
            type="button"
            class="remove"
            aria-label="Remove image"
            onclick={() => side.images.splice(i, 1)}>×</button
          >
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .side {
    min-width: 0;
  }

  label {
    font-weight: 600;
  }

  textarea {
    width: 100%;
    resize: vertical;
  }

  .error {
    margin: 0;
  }

  .previews {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
  }

  .tile {
    position: relative;
    max-width: 100%;
    border: 1px solid var(--border);
    border-radius: 6px;
    padding: 4px;
    background: var(--surface);
  }

  img {
    display: block;
    max-width: 100%;
    max-height: 220px;
    object-fit: contain;
  }

  .remove {
    position: absolute;
    top: 6px;
    right: 6px;
    width: 26px;
    height: 26px;
    padding: 0;
    justify-content: center;
    border-radius: 50%;
    font-size: 16px;
    line-height: 1;
  }
</style>
