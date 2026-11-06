<script lang="ts">
  import { mediaSrc } from '$lib/media';
  import type { CardSide } from '$lib/types';

  let { side, size }: { side: CardSide; size: 'thumb' | 'full' } = $props();
</script>

<div class="face {size}">
  {#if side.text.trim() !== ''}
    <div class="text">{side.text}</div>
  {/if}
  {#if side.images.length > 0}
    <div class="images">
      {#each side.images as name (name)}
        <img src={mediaSrc(name)} alt="" />
      {/each}
    </div>
  {/if}
</div>

<style>
  .face {
    display: flex;
    flex-direction: column;
    gap: 8px;
    min-width: 0;
  }

  .text {
    white-space: pre-wrap;
    overflow-wrap: anywhere;
  }

  .images {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
  }

  img {
    max-width: 100%;
    object-fit: contain;
  }

  .thumb .text {
    display: -webkit-box;
    -webkit-line-clamp: 3;
    line-clamp: 3;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }

  .thumb img {
    max-height: 60px;
  }

  .full {
    align-items: center;
  }

  .full .text {
    font-size: 1.5rem;
    text-align: center;
  }

  .full .images {
    justify-content: center;
  }

  .full img {
    max-height: 50vh;
  }
</style>
