// Tauri doesn't have a Node.js server to do proper SSR
// so we use adapter-static with a fallback to index.html to put the site in SPA mode
// See: https://svelte.dev/docs/kit/single-page-apps
// See: https://v2.tauri.app/start/frontend/sveltekit/ for more info
import { initMedia } from '$lib/media';

export const ssr = false;

// Resolve the media dir once so `mediaSrc()` stays synchronous everywhere.
export const load = async () => {
  await initMedia();
  return {};
};
