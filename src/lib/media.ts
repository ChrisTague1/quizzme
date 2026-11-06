import { convertFileSrc } from '@tauri-apps/api/core';
import * as api from './api';

/** Absolute media dir with trailing separator; set once by the root layout load. */
let dir = '';

export async function initMedia() {
  if (dir) return;
  dir = await api.mediaDir();
}

export function mediaSrc(name: string): string {
  return convertFileSrc(dir + name);
}

const MIME_EXT: Record<string, string> = {
  'image/png': 'png',
  'image/jpeg': 'jpg',
  'image/gif': 'gif',
  'image/webp': 'webp',
  'image/bmp': 'bmp',
  'image/tiff': 'tiff',
};

/** Stores an image in the media dir and returns its file name. */
export async function storeImage(file: File): Promise<string> {
  const ext = MIME_EXT[file.type];
  if (!ext) throw new Error(`Unsupported image type: ${file.type || 'unknown'}`);
  return api.saveMedia(new Uint8Array(await file.arrayBuffer()), ext);
}
