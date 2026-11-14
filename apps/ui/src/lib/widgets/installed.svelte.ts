import { invoke } from '@tauri-apps/api/core';
import { onWidgetsChanged } from '#lib/tauri/events.ts';
import type { InstalledWidget } from './types';

let list = $state.raw<InstalledWidget[]>([]);
let scanError = $state<string>();

/** Reads the widget folders again. A failure keeps the previous list and sets `installed.error`. */
export async function rescanWidgets() {
	try {
		const found = await invoke<InstalledWidget[]>('widgets_list');
		list = found.sort((a, b) => a.name.localeCompare(b.name));
		scanError = undefined;
	} catch (error) {
		scanError = String(error);
	}
}

export const openWidgetsFolder = () => invoke('widgets_open_dir').catch(console.error);

/** The installed widgets. The scan starts on the first import, in the settings window only. */
export const installed = {
	get current() {
		return list;
	},
	/** Why the last scan failed. */
	get error() {
		return scanError;
	}
};

export const widgetName = (id: string) => list.find((widget) => widget.id === id)?.name ?? id;

void rescanWidgets();
onWidgetsChanged(() => void rescanWidgets());
