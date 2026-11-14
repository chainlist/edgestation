import { onWidgetsChanged } from '#lib/tauri/events.ts';
import { forgetWidget } from './loader';

const revisions: Record<string, number> = $state({});

/**
 * Counts how many times the marketplace replaced or removed the files of widget `id` since the
 * window opened. Each placement uses it as a key (`WidgetSlot.svelte`), so a change remounts it.
 */
export const widgetRevision = (id: string) => revisions[id] ?? 0;

/**
 * For the Edge window. When the marketplace installs, updates or removes widgets, this forgets
 * their loaded modules and remounts their placements. The other widgets stay as they are.
 * Returns the function that stops this.
 */
export const watchWidgetChanges = () =>
	onWidgetsChanged((ids) => {
		for (const id of ids) {
			forgetWidget(id);
			revisions[id] = widgetRevision(id) + 1;
		}
	});
