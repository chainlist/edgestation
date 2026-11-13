import { settingsReady } from '#lib/state/settings.svelte.ts';

// Tauri serves static files, so render all pages on the client.
export const ssr = false;

// Read the saved settings before the first render, so no window shows the defaults for a moment.
export const load = async () => {
	await settingsReady.catch((error) => console.error('Could not read settings', error));
};
