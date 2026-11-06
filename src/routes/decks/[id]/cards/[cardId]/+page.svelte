<script lang="ts">
  import { page } from '$app/state';
  import CardEditor from '$lib/components/CardEditor.svelte';
  import type { PageProps } from './$types';

  let { data }: PageProps = $props();
</script>

<a class="breadcrumb" href="/decks/{data.deck.id}">← {data.deck.title}</a>
<h1>{data.card ? 'Edit card' : 'New card'}</h1>

{#key page.params.cardId}
  <CardEditor deckId={data.deck.id} card={data.card} />
{/key}

<style>
  h1 {
    margin-bottom: 16px;
  }
</style>
