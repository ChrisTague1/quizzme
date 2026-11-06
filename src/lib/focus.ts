import type { Attachment } from 'svelte/attachments';

/** `{@attach autofocus}`: focuses the element when mounted (avoids the a11y-flagged `autofocus` attribute). */
export const autofocus: Attachment<HTMLElement> = (node) => {
  node.focus();
};
