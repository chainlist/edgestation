import { invoke } from '@tauri-apps/api/core';
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';

/**
 * Calls `fn` with true if the window of another app is full screen, first immediately and then
 * after each change. Returns the function that stops the watch.
 */
export function watchFullscreenApp(fn: (fullscreen: boolean) => void): () => void {
	const listening = getCurrentWebviewWindow().listen<boolean>('fullscreen-app', (event) =>
		fn(event.payload)
	);
	// Listen first, because the host sends the current state when the watch starts.
	listening.then(() => invoke('watch_fullscreen', { enabled: true })).catch(console.error);
	return () => {
		void listening.then((off) => off());
		invoke('watch_fullscreen', { enabled: false }).catch(console.error);
	};
}
