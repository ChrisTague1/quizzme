import { listDecks } from '$lib/api';
import type { PageLoad } from './$types';

export const load: PageLoad = async () => ({ decks: await listDecks() });
