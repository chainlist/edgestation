import { check, type Update } from '@tauri-apps/plugin-updater';
import { installUpdate } from '#lib/tauri/updater.ts';

/**
 * One offer of a newer release of EdgeStation: the release found, its download and its error.
 * The Edge prompt and Settings > General each make one. Nothing downloads until `install`.
 */
export function createAppUpdate() {
	let update = $state<Update>();
	/** True while a check or a download runs. */
	let busy = $state(false);
	/** The part of the download that is done, from 0 to 100. */
	let percent = $state(0);
	let error = $state<string>();

	return {
		/** The newer release that the last check found, or `undefined`. */
		get update() {
			return update;
		},
		get busy() {
			return busy;
		},
		get error() {
			return error;
		},
		/** The label of the install button, with the progress of the download. */
		get installLabel() {
			return busy && update ? `Downloading… ${percent}%` : 'Download & install';
		},
		/** Looks for a newer release. Resolves to it, or to `undefined` if none or on error. */
		async check() {
			busy = true;
			error = undefined;
			try {
				update = (await check()) ?? undefined;
				return update;
			} catch (e) {
				console.error(e);
				error = `Couldn't update: ${e}`;
			} finally {
				busy = false;
			}
		},
		/** Downloads and installs the release found. Resolves to `false` if it failed. */
		async install() {
			if (!update) return false;
			busy = true;
			percent = 0;
			error = undefined;
			try {
				await installUpdate(update, (p) => (percent = p));
				return true;
			} catch (e) {
				console.error(e);
				error = `Couldn't update: ${e}`;
				return false;
			} finally {
				busy = false;
			}
		},
		/** Forgets the release found, until the next check. */
		dismiss() {
			update?.close().catch(console.error);
			update = undefined;
		}
	};
}
