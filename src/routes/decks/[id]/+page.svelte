<script lang="ts">
  import { goto, invalidateAll } from '$app/navigation';
  import { deleteDeck, errorMessage, updateDeck } from '$lib/api';
  import CardFace from '$lib/components/CardFace.svelte';
  import ConfirmButton from '$lib/components/ConfirmButton.svelte';
  import { autofocus } from '$lib/focus';
  import type { PageProps } from './$types';

  let { data }: PageProps = $props();

  let editing = $state(false);
  let title = $state('');
  let description = $state('');
  let busy = $state(false);
  let error = $state('');

  function toggleEdit() {
    if (!editing) {
      title = data.deck.title;
      description = data.deck.description;
    }
    error = '';
    editing = !editing;
  }

  async function save(e: SubmitEvent) {
    e.preventDefault();
    busy = true;
    error = '';
    try {
      await updateDeck(data.deck.id, title, description);
      await invalidateAll();
      editing = false;
    } catch (err) {
      error = errorMessage(err);
    } finally {
      busy = false;
    }
  }

  async function remove() {
    error = '';
    try {
      await deleteDeck(data.deck.id);
      await goto('/');
    } catch (err) {
      error = errorMessage(err);
    }
  }
</script>

<div class="stack">
  <a class="breadcrumb" href="/">← Study guides</a>

  <div class="stack header">
    <h1>{data.deck.title}</h1>
    {#if data.deck.description}<p class="desc">{data.deck.description}</p>{/if}
    <span class="muted">{data.cards.length} {data.cards.length === 1 ? 'card' : 'cards'}</span>
  </div>

  <div class="row">
    {#if data.cards.length === 0}
      <button type="button" class="primary" disabled title="Add a card first">Study</button>
    {:else}
      <a class="button primary" href="/decks/{data.deck.id}/study">Study</a>
    {/if}
    <a class="button" href="/decks/{data.deck.id}/cards/new">Add card</a>
    <button type="button" onclick={toggleEdit}>Edit details</button>
    <span class="spacer"></span>
    <ConfirmButton label="Delete study guide" onconfirm={remove} />
  </div>
  {#if error && !editing}<p class="error">{error}</p>{/if}

  {#if editing}
    <form class="panel stack" onsubmit={save}>
      <input bind:value={title} placeholder="Title" aria-label="Title" {@attach autofocus} />
      <textarea
        bind:value={description}
        placeholder="Description (optional)"
        aria-label="Description"
        rows="3"
      ></textarea>
      <div class="row">
        <button type="submit" class="primary" disabled={busy || title.trim() === ''}>Save</button>
        <button type="button" onclick={toggleEdit}>Cancel</button>
        {#if error}<span class="error">{error}</span>{/if}
      </div>
    </form>
  {/if}

  {#if data.cards.length === 0}
    <div class="panel empty">
      <p class="muted">No cards yet.</p>
      <a href="/decks/{data.deck.id}/cards/new">Add card</a>
    </div>
  {:else}
    <div class="cards">
      {#each data.cards as card (card.id)}
        <a class="panel card-row" href="/decks/{data.deck.id}/cards/{card.id}">
          <CardFace side={card.front} size="thumb" />
          <CardFace side={card.back} size="thumb" />
        </a>
      {/each}
    </div>
  {/if}
</div>

<style>
  .header {
    gap: 4px;
  }

  .desc {
    margin: 0;
    white-space: pre-wrap;
  }

  .empty p {
    margin: 0 0 8px;
  }

  .cards {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .card-row {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 24px;
    color: var(--fg);
  }

  .card-row:hover {
    border-color: var(--accent);
  }
</style>
