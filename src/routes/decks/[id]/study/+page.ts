import { loadDeckWithCards } from '$lib/load';
import type { PageLoad } from './$types';

export const load: PageLoad = ({ params }) => loadDeckWithCards(params.id);
