import { invoke } from '@tauri-apps/api/core';
import { onWidgetsChanged } from '#lib/tauri/events.ts';
import type { InstalledWidget } from './types';

let list = $state.raw<InstalledWidget[]>([]);

export async function rescanWidgets() {
	const found = await invoke<InstalledWidget[]>('list_widgets');
	list = found.sort((a, b) => a.name.localeCompare(b.name));
}

export const openWidgetsFolder = () => invoke('open_widgets_dir');

/** The installed widgets. The scan starts on the first import, in the settings window only. */
export const installed = {
	get current() {
		return list;
	}
};

export const widgetName = (id: string) => list.find((widget) => widget.id === id)?.name ?? id;

void rescanWidgets();
void onWidgetsChanged(() => void rescanWidgets());
