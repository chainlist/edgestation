import { getVersion } from '@tauri-apps/api/app';
import { invoke } from '@tauri-apps/api/core';
import type { ListedWidget } from '../types';

let listing = $state.raw<ListedWidget[]>();
let listingError = $state<string>();
let loading: Promise<void> | undefined;
let appVersion = $state<string>();
getVersion().then((version) => (appVersion = version), console.error);

/** For each widget id, the action that runs now: install or uninstall. */
const busy: Record<string, 'install' | 'uninstall'> = $state({});
/** For each widget id, why its last install, update or removal failed. */
const errors: Record<string, string> = $state({});

/** The marketplace from the last fecth, in the settings window only. */
export const marketplace = {
	/** All marketplace widgets, `undefined` until the fetch is done. */
	get listing() {
		return listing;
	},
	/** Why the fetch of the marketplace failed. */
	get error() {
		return listingError;
	},
	get appVersion() {
		return appVersion;
	},
	busy: (id: string) => busy[id],
	errorOf: (id: string) => errors[id],
	/** Why the last action on installed widget `id` failed, or that the marketplace delisted it. */
	problemOf: (id: string) =>
		errors[id] ??
		(listing && !listing.some((listed) => listed.id === id)
			? 'No longer in the marketplace, so it no longer gets updates.'
			: undefined)
};

/** Fetches the marketplace one time per session, or again with `refresh`. */
export function loadListing(refresh = false): Promise<void> {
	if (!loading || refresh) {
		listingError = undefined;
		loading = invoke<ListedWidget[]>('marketplace_list').then(
			(found) => void (listing = found),
			(error) => {
				listingError = String(error);
				loading = undefined;
			}
		);
	}
	return loading;
}

async function run(id: string, action: 'install' | 'uninstall') {
	busy[id] = action;
	delete errors[id];
	try {
		await invoke(`marketplace_${action}`, { id });
	} catch (error) {
		errors[id] = String(error);
	} finally {
		delete busy[id];
	}
}

/** Installs marketplace widget `id`, or updates it to its latest release. */
export const installWidget = (id: string) => run(id, 'install');

export const uninstallWidget = (id: string) => run(id, 'uninstall');

/**
 * Installs all newer releases of the installed marketplace widgets (Update all). The widgets
 * `ids` show as busy meanwhile. Each failure shows under its widget. Rejects if the host cannot
 * look for releases at all.
 */
export async function updateWidgets(ids: string[]): Promise<void> {
	for (const id of ids) {
		busy[id] = 'install';
		delete errors[id];
	}
	try {
		const attempts = await invoke<{ id: string; error: string | null }[]>('marketplace_update');
		for (const { id, error } of attempts) {
			if (error) errors[id] = error;
			else delete errors[id];
		}
	} finally {
		for (const id of ids) delete busy[id];
	}
}

export const openLink = (url: string) => invoke('open_link', { url });
