import { error } from '@sveltejs/kit';
import { getDeck, listCards } from './api';

export function parseId(raw: string, notFound: string): number {
  const id = Number(raw);
  if (!Number.isInteger(id)) error(404, notFound);
  return id;
}

export async function loadDeckWithCards(rawId: string) {
  const id = parseId(rawId, 'Study guide not found');
  const [deck, cards] = await Promise.all([getDeck(id), listCards(id)]).catch((e) =>
    error(404, String(e)),
  );
  return { deck, cards };
}
