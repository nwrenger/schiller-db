import { writable } from 'svelte/store';

export const dialog = writable<null | HTMLDivElement>(null);
export const staticBackdropLabel = writable<null | HTMLHeadingElement>(null);
export const modalBody = writable<null | HTMLDivElement>(null);

export const sidebarState = writable<null | String>(null);
export const containerState = writable<null | String>(null);