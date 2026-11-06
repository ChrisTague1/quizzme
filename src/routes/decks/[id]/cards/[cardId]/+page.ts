import { error } from '@sveltejs/kit';
import { getCard, getDeck } from '$lib/api';
import { parseId } from '$lib/load';
import type { Card } from '$lib/types';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params }) => {
  const deckId = parseId(params.id, 'Study guide not found');
  let card: Card | null = null;
  if (params.cardId !== 'new') {
    const cardId = parseId(params.cardId, 'Card not found');
    card = await getCard(cardId).catch((e) => error(404, String(e)));
    if (card.deckId !== deckId) error(404, 'Card not found');
  }
  const deck = await getDeck(deckId).catch((e) => error(404, String(e)));
  return { deck, card };
};
