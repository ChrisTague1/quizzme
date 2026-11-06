<script lang="ts">
  import { untrack } from 'svelte';
  import { goto } from '$app/navigation';
  import { deleteCard, errorMessage, saveCard } from '$lib/api';
  import type { Card, CardSide } from '$lib/types';
  import CardSideEditor from './CardSideEditor.svelte';
  import ConfirmButton from './ConfirmButton.svelte';

  let { deckId, card }: { deckId: number; card: Card | null } = $props();

  const empty = (): CardSide => ({ text: '', images: [] });
  const nonEmpty = (s: CardSide) => s.text.trim() !== '' || s.images.length > 0;

  // The parent re-keys this component per card, so capturing the initial prop is intended.
  const initial = untrack(() => card);
  let front = $state(structuredClone(initial?.front ?? empty()));
  let back = $state(structuredClone(initial?.back ?? empty()));
  let frontEditor: CardSideEditor;
  let busy = $state(false);
  let error = $state('');

  const valid = $derived(nonEmpty(front) && nonEmpty(back));
  const deckUrl = $derived(`/decks/${deckId}`);

  async function persist(): Promise<boolean> {
    busy = true;
    error = '';
    try {
      await saveCard({
        id: card?.id ?? null,
        deckId,
        front: $state.snapshot(front),
        back: $state.snapshot(back),
      });
      return true;
    } catch (e) {
      error = errorMessage(e);
      return false;
    } finally {
      busy = false;
    }
  }

  async function save() {
    if (await persist()) await goto(deckUrl);
  }

  async function saveAndAddAnother() {
    if (!(await persist())) return;
    front = empty();
    back = empty();
    frontEditor.focus();
  }

  async function remove() {
    if (!card) return;
    error = '';
    try {
      await deleteCard(card.id);
      await goto(deckUrl);
    } catch (e) {
      error = errorMessage(e);
    }
  }
</script>

<div class="stack">
  <div class="sides">
    <CardSideEditor bind:this={frontEditor} label="Front" bind:side={front} autofocus />
    <CardSideEditor label="Back" bind:side={back} />
  </div>

  <div class="row">
    <button type="button" class="primary" disabled={!valid || busy} onclick={save}>Save</button>
    {#if card === null}
      <button type="button" disabled={!valid || busy} onclick={saveAndAddAnother}>
        Save &amp; add another
      </button>
    {/if}
    <button type="button" onclick={() => goto(deckUrl)}>Cancel</button>
    <span class="spacer"></span>
    {#if card !== null}
      <ConfirmButton label="Delete card" onconfirm={remove} />
    {/if}
  </div>
  {#if !valid}<p class="muted hint">Each side needs text or an image.</p>{/if}
  {#if error}<p class="error hint">{error}</p>{/if}
</div>

<style>
  .sides {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 20px;
  }

  @media (max-width: 800px) {
    .sides {
      grid-template-columns: 1fr;
    }
  }

  .hint {
    margin: 0;
  }
</style>
