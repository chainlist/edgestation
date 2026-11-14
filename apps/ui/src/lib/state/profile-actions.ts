import { invoke } from '@tauri-apps/api/core';
import { leaveProfile } from '#lib/layout/pages.ts';
import { newId, removeIn, updateIn } from '#lib/lists.ts';
import { pages, profiles } from './settings.svelte';

export function addProfile() {
	const profile = { id: newId(), name: `Profile ${profiles.current.length + 1}` };
	profiles.current = [...profiles.current, profile];
}

export function renameProfile(id: string, name: string) {
	profiles.current = updateIn(profiles.current, id, (profile) => ({ ...profile, name }));
}

/**
 * Deletes profile `id` and its data, which signs the profile out of all sites. Then it moves the
 * widgets that used the profile to Default. The host closes their views first. If the host
 * cannot delete the data, nothing changes and the promise rejects.
 */
export async function deleteProfile(id: string) {
	await invoke('webview_profile_delete', { profile: id });
	pages.current = leaveProfile(pages.current, id);
	profiles.current = removeIn(profiles.current, id);
}
