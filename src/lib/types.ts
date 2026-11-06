export interface Deck {
  id: number;
  title: string;
  description: string;
  createdAt: number;
  updatedAt: number;
  cardCount: number;
}

export interface CardSide {
  text: string;
  images: string[];
}

export interface Card {
  id: number;
  deckId: number;
  front: CardSide;
  back: CardSide;
  createdAt: number;
  updatedAt: number;
}

export interface CardInput {
  id: number | null;
  deckId: number;
  front: CardSide;
  back: CardSide;
}

export type StartSide = 'front' | 'back' | 'random';
