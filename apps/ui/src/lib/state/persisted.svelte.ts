import { settingsStore } from '#lib/tauri/store.ts';

export interface Persisted<T> {
	current: T;
	/** Resolves when the saved value is read, or when there is no saved value. */
	readonly ready: Promise<void>;
}

/**
 * A reactive settings value that the app saves to disk. All windows see the same value.
 * Do not change a value in place. Assign a new object to `current` to save it.
 */
export function persisted<T>(key: string, fallback: T): Persisted<T> {
	let value = $state.raw(fallback);
	// Each write from this window comes back as a change event. Ignore these echoes, or a slider
	// jumps back when an older echo arives after a newer local value.
	let echoes = 0;

	const ready = settingsStore.get<T>(key).then((saved) => {
		if (saved !== undefined) value = saved;
	});
	void settingsStore.onKeyChange<T>(key, (next) => {
		if (echoes > 0) echoes--;
		else value = next ?? fallback;
	});

	return {
		ready,
		get current() {
			return value;
		},
		set current(next: T) {
			value = next;
			echoes++;
			settingsStore.set(key, next).catch((error) => {
				echoes--;
				console.error(`Could not save "${key}"`, error);
			});
		}
	};
}

/** Saves a shallow update of an object setting. */
export function patch<T extends object>(target: Persisted<T>, changes: Partial<T>) {
	target.current = { ...target.current, ...changes };
}
