import type { HandleClientError } from '@sveltejs/kit/hooks';

// SvelteKit sends each error that a <svelte:boundary> catches through this hook. By default it
// replaces the mesage with "Internal Error". Keep the real message, so the error card of a
// widget can show what went wrong.
export const handleError: HandleClientError = ({ kind, error }) => {
	if (kind !== 'unknown') return;
	console.error(error);
	return { message: error instanceof Error ? error.message : String(error) };
};
