<script lang="ts">
  import { goto } from '$app/navigation';
  import { createDeck, errorMessage, openDataDir } from '$lib/api';
  import { autofocus } from '$lib/focus';
  import type { Deck } from '$lib/types';
  import type { PageProps } from './$types';

  let { data }: PageProps = $props();

  type SortKey = 'updated' | 'name-asc' | 'name-desc' | 'created' | 'cards';

  const byName = (a: Deck, b: Deck) =>
    a.title.localeCompare(b.title, undefined, { sensitivity: 'base' });

  const SORTS: { value: SortKey; label: string; compare: (a: Deck, b: Deck) => number }[] = [
    { value: 'updated', label: 'Recently updated', compare: (a, b) => b.updatedAt - a.updatedAt },
    { value: 'name-asc', label: 'Name A–Z', compare: byName },
    { value: 'name-desc', label: 'Name Z–A', compare: (a, b) => byName(b, a) },
    { value: 'created', label: 'Newest', compare: (a, b) => b.createdAt - a.createdAt },
    { value: 'cards', label: 'Most cards', compare: (a, b) => b.cardCount - a.cardCount },
  ];

  const SORT_STORAGE = 'quizzme.deckSort';

  function storedSort(): SortKey {
    const v = localStorage.getItem(SORT_STORAGE);
    return SORTS.find((s) => s.value === v)?.value ?? 'updated';
  }

  let sort = $state<SortKey>(storedSort());
  let query = $state('');

  const visible = $derived.by(() => {
    const q = query.trim().toLowerCase();
    const compare = SORTS.find((s) => s.value === sort)!.compare;
    return data.decks
      .filter((d) => !q || `${d.title} ${d.description}`.toLowerCase().includes(q))
      .sort(compare);
  });

  let creating = $state(false);
  let title = $state('');
  let description = $state('');
  let createError = $state('');
  let busy = $state(false);
  let pageError = $state('');

  function startCreate() {
    title = '';
    description = '';
    createError = '';
    creating = true;
  }

  async function create(e: SubmitEvent) {
    e.preventDefault();
    busy = true;
    createError = '';
    try {
      const id = await createDeck(title, description);
      await goto(`/decks/${id}`);
    } catch (err) {
      createError = errorMessage(err);
    } finally {
      busy = false;
    }
  }

  async function openFolder() {
    pageError = '';
    try {
      await openDataDir();
    } catch (err) {
      pageError = errorMessage(err);
    }
  }
</script>

<div class="stack">
  <div class="row">
    <h1>Study guides</h1>
    <span class="spacer"></span>
    <button type="button" onclick={openFolder}>Open data folder</button>
    <button type="button" class="primary" onclick={startCreate} disabled={creating}>
      New study guide
    </button>
  </div>
  {#if pageError}<p class="error">{pageError}</p>{/if}

  {#if creating}
    <form class="panel stack" onsubmit={create}>
      <input bind:value={title} placeholder="Title" aria-label="Title" {@attach autofocus} />
      <textarea
        bind:value={description}
        placeholder="Description (optional)"
        aria-label="Description"
        rows="3"
      ></textarea>
      <div class="row">
        <button type="submit" class="primary" disabled={busy || title.trim() === ''}>Create</button>
        <button type="button" onclick={() => (creating = false)}>Cancel</button>
        {#if createError}<span class="error">{createError}</span>{/if}
      </div>
    </form>
  {/if}

  {#if data.decks.length === 0}
    <p class="muted">No study guides yet. Create one to get started.</p>
  {:else}
    <div class="row">
      <input
        class="search"
        type="search"
        bind:value={query}
        placeholder="Search study guides…"
        aria-label="Search study guides"
      />
      <select
        bind:value={sort}
        aria-label="Sort"
        onchange={() => localStorage.setItem(SORT_STORAGE, sort)}
      >
        {#each SORTS as s (s.value)}
          <option value={s.value}>{s.label}</option>
        {/each}
      </select>
    </div>

    {#if visible.length === 0}
      <p class="muted">No study guides match "{query}".</p>
    {:else}
      <div class="grid">
        {#each visible as deck (deck.id)}
          <a class="panel tile" href="/decks/{deck.id}">
            <strong class="title">{deck.title}</strong>
            {#if deck.description}<span class="desc muted">{deck.description}</span>{/if}
            <span class="meta muted">
              {deck.cardCount}
              {deck.cardCount === 1 ? 'card' : 'cards'} · Updated {new Date(
                deck.updatedAt,
              ).toLocaleDateString()}
            </span>
          </a>
        {/each}
      </div>
    {/if}
  {/if}
</div>

<style>
  .search {
    flex: 1;
    min-width: 200px;
  }

  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(240px, 1fr));
    gap: 12px;
  }

  .tile {
    display: flex;
    flex-direction: column;
    gap: 6px;
    color: var(--fg);
    min-height: 110px;
  }

  .tile:hover {
    border-color: var(--accent);
  }

  .title {
    font-size: 1.05rem;
    overflow-wrap: anywhere;
  }

  .desc {
    display: -webkit-box;
    -webkit-line-clamp: 2;
    line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }

  .meta {
    margin-top: auto;
    font-size: 0.85rem;
  }
</style>
