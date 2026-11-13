import { register, unregisterAll } from '@tauri-apps/plugin-global-shortcut';
import type { Shortcut } from '#lib/shortcuts/keys.ts';

/** Each change waits for the previous change, so an old set cannot replace a newer set. */
let queue: Promise<void> = Promise.resolve();

/**
 * Registers the shortcut of each action for the whole system, until you call the returned
 * function. When the user presses the keys of a shortcut, `run` gets its action. `failed` gets
 * the action whose shortcut did not register, usually because another app has it.
 */
export function bindShortcuts<A extends string>(
	shortcuts: Record<A, Shortcut | null>,
	run: (action: A) => void,
	failed: (actions: A[]) => void
): () => void {
	let bound = true;
	queue = queue
		.then(async () => {
			// This also removes the shortcuts of an earlier load of this window, because a reload
			// does not clean up.
			await unregisterAll();
			if (!bound) return;
			const failures: A[] = [];
			for (const [action, shortcut] of Object.entries(shortcuts) as [A, Shortcut | null][]) {
				if (!shortcut) continue;
				await register(shortcut, (e) => {
					if (bound && e.state === 'Pressed') run(action);
				}).catch((error) => {
					console.error(`Could not register the shortcut ${shortcut}`, error);
					failures.push(action);
				});
			}
			if (bound) failed(failures);
		})
		.catch(console.error);
	return () => {
		bound = false;
		queue = queue.then(unregisterAll).catch(console.error);
	};
}
