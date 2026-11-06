import { invoke } from '@tauri-apps/api/core';
import type { Card, CardInput, Deck } from './types';

/** Rust commands reject with plain strings; JS failures throw Errors. */
export function errorMessage(e: unknown): string {
  return e instanceof Error ? e.message : String(e);
}

export const listDecks = () => invoke<Deck[]>('list_decks');
export const getDeck = (id: number) => invoke<Deck>('get_deck', { id });
export const createDeck = (title: string, description: string) =>
  invoke<number>('create_deck', { title, description });
export const updateDeck = (id: number, title: string, description: string) =>
  invoke<void>('update_deck', { id, title, description });
export const deleteDeck = (id: number) => invoke<void>('delete_deck', { id });

export const listCards = (deckId: number) => invoke<Card[]>('list_cards', { deckId });
export const getCard = (id: number) => invoke<Card>('get_card', { id });
export const saveCard = (card: CardInput) => invoke<number>('save_card', { card });
export const deleteCard = (id: number) => invoke<void>('delete_card', { id });

/** Raw-body IPC: bytes go as the request body, extension as the `x-ext` header. */
export const saveMedia = (bytes: Uint8Array, ext: string) =>
  invoke<string>('save_media', bytes, { headers: { 'x-ext': ext } });
export const mediaDir = () => invoke<string>('media_dir');
export const openDataDir = () => invoke<void>('open_data_dir');
