# Quizzme

Desktop flashcard app (Tauri 2 + SvelteKit SPA, Svelte 5, Bun).

- Study guides (decks): create, edit, delete, search, sort.
- Cards: front/back, each with text and/or images. Paste an image (⌘V) into a side's text box, or use "Add image…".
- Study mode: cards always shuffled; start side Front (default), Back or Random. Space/Enter flips, ←/→ move.

## Data

`~/Library/Application Support/com.christague.quizzme/`

- `quizzme.db`: SQLite (decks, cards; image lists stored as JSON arrays of file names).
- `media/`: pasted images (`<uuid>.<ext>`; png, jpg, gif, webp, bmp, tiff).

Unreferenced media files (from cancelled edits, removed images, deleted cards/decks) are deleted on the next app start, never mid-session.

## Development

```sh
bun install
bun run tauri dev      # run the app
bun run check          # svelte-check / TypeScript
cd src-tauri && cargo test   # storage-layer tests
bun run tauri build    # release bundle in src-tauri/target/release/bundle/
```
