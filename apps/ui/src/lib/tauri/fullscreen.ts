import { invoke } from '@tauri-apps/api/core';
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
import { cleanupOf } from './events';

/**
 * Calls `fn` with true if the window of another app is full screen, first immediately and then
 * after each change. Returns the function that stops the watch.
 */
export function watchFullscreenApp(fn: (fullscreen: boolean) => void): () => void {
	const listening = getCurrentWebviewWindow().listen<boolean>('fullscreen:changed', (event) =>
		fn(event.payload)
	);
	// Listen first, because the host sends the current state when the watch starts.
	listening.then(() => invoke('fullscreen_watch', { on: true })).catch(console.error);
	const stopListening = cleanupOf(listening);
	return () => {
		stopListening();
		invoke('fullscreen_watch', { on: false }).catch(console.error);
	};
}
