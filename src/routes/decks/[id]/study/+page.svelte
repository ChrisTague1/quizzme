<script lang="ts">
  import { onMount } from 'svelte';
  import CardFace from '$lib/components/CardFace.svelte';
  import type { Card, StartSide } from '$lib/types';
  import type { PageProps } from './$types';

  let { data }: PageProps = $props();

  type Side = 'front' | 'back';

  const START_STORAGE = 'quizzme.startSide';
  const START_OPTIONS: { value: StartSide; label: string }[] = [
    { value: 'front', label: 'Front' },
    { value: 'back', label: 'Back' },
    { value: 'random', label: 'Random' },
  ];

  function storedStartSide(): StartSide {
    const v = localStorage.getItem(START_STORAGE);
    return START_OPTIONS.find((o) => o.value === v)?.value ?? 'front';
  }

  let startSide = $state<StartSide>(storedStartSide());
  let order = $state.raw<Card[]>([]);
  let starts = $state.raw<Side[]>([]);
  let index = $state(0);
  let flipped = $state(false);
  let done = $state(false);

  const other = (s: Side): Side => (s === 'front' ? 'back' : 'front');
  const current = $derived(order[index]);
  const shown = $derived<Side>(flipped ? other(starts[index]) : starts[index]);

  function shuffleSession() {
    const cards = [...data.cards];
    for (let i = cards.length - 1; i > 0; i--) {
      const j = Math.floor(Math.random() * (i + 1));
      [cards[i], cards[j]] = [cards[j], cards[i]];
    }
    order = cards;
    starts = cards.map(() =>
      startSide === 'random' ? (Math.random() < 0.5 ? 'front' : 'back') : startSide,
    );
    index = 0;
    flipped = false;
    done = false;
  }

  onMount(shuffleSession);

  function setStartSide(value: StartSide) {
    startSide = value;
    localStorage.setItem(START_STORAGE, value);
    shuffleSession();
  }

  function go(delta: number) {
    const next = index + delta;
    if (next < 0) return;
    if (next >= order.length) {
      done = true;
      return;
    }
    index = next;
    flipped = false;
  }

  /** Blur so Space/Enter shortcuts don't re-activate the clicked button. */
  function blurred(action: () => void) {
    return (e: MouseEvent) => {
      (e.currentTarget as HTMLElement).blur();
      action();
    };
  }

  function onkeydown(e: KeyboardEvent) {
    if (done || order.length === 0) return;
    const target = e.target instanceof Element ? e.target : null;
    if (target?.closest('input, select, textarea')) return;
    if (e.key === ' ' || e.key === 'Enter') {
      if (target?.closest('button')) return;
      e.preventDefault();
      flipped = !flipped;
    } else if (e.key === 'ArrowRight') {
      go(1);
    } else if (e.key === 'ArrowLeft') {
      go(-1);
    }
  }
</script>

<svelte:window {onkeydown} />

<a class="breadcrumb" href="/decks/{data.deck.id}">← {data.deck.title}</a>

{#if data.cards.length === 0}
  <div class="panel">
    <p class="muted">This study guide has no cards yet.</p>
    <a href="/decks/{data.deck.id}/cards/new">Add a card</a>
  </div>
{:else}
  <div class="stack">
    <div class="row">
      <h1>{data.deck.title}</h1>
      <span class="spacer"></span>
      <div class="segmented" role="radiogroup" aria-label="Start with">
        <span class="muted">Start with:</span>
        {#each START_OPTIONS as opt (opt.value)}
          <label class:active={startSide === opt.value}>
            <input
              type="radio"
              name="start-side"
              value={opt.value}
              checked={startSide === opt.value}
              onchange={(e) => {
                // A focused radio would swallow ←/→ (native radio navigation) instead of moving cards.
                e.currentTarget.blur();
                setStartSide(opt.value);
              }}
            />
            {opt.label}
          </label>
        {/each}
      </div>
    </div>

    {#if done}
      <div class="panel done stack">
        <p>You went through all {order.length} cards.</p>
        <div class="row">
          <button type="button" class="primary" onclick={blurred(shuffleSession)}>
            Shuffle again
          </button>
          <a class="button" href="/decks/{data.deck.id}">Back to study guide</a>
        </div>
      </div>
    {:else if current}
      <button type="button" class="panel card" onclick={() => (flipped = !flipped)}>
        <span class="side-label muted">{shown === 'front' ? 'Front' : 'Back'}</span>
        <CardFace side={current[shown]} size="full" />
      </button>

      <div class="row controls">
        <button type="button" disabled={index === 0} onclick={blurred(() => go(-1))}>
          ← Previous
        </button>
        <span class="muted">{index + 1} / {order.length}</span>
        <button type="button" onclick={blurred(() => go(1))}>Next →</button>
      </div>
      <p class="muted keys">Space / Enter to flip · ← → to move</p>
    {/if}
  </div>
{/if}

<style>
  .segmented {
    display: inline-flex;
    align-items: center;
    gap: 4px;
  }

  .segmented label {
    padding: 5px 12px;
    border: 1px solid var(--border);
    border-radius: 6px;
    background: var(--surface);
    cursor: pointer;
  }

  .segmented label.active {
    color: var(--accent-fg);
    background: var(--accent);
    border-color: var(--accent);
  }

  .segmented input {
    position: absolute;
    opacity: 0;
    pointer-events: none;
  }

  .segmented label:has(input:focus-visible) {
    outline: 2px solid var(--accent);
    outline-offset: 2px;
  }

  .card {
    position: relative;
    display: flex;
    flex-direction: column;
    justify-content: center;
    width: 100%;
    min-height: 420px;
    padding: 48px 32px;
    white-space: normal;
    text-align: center;
  }

  .side-label {
    position: absolute;
    top: 12px;
    left: 16px;
    font-size: 0.8rem;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .controls {
    justify-content: center;
    gap: 16px;
  }

  .keys {
    margin: 0;
    text-align: center;
    font-size: 0.85rem;
  }

  .done p {
    margin: 0;
    font-size: 1.15rem;
  }
</style>
