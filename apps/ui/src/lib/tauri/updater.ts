import type { Update } from '@tauri-apps/plugin-updater';

/**
 * Downloads and installs `update`. It gives the part of the download that is done, from 0 to
 * 100, to `progress`. On Windows the app quits as the installer starts. On Linux the new version
 * runs at the next launch.
 */
export async function installUpdate(update: Update, progress: (percent: number) => void) {
	let total = 0;
	let done = 0;
	await update.downloadAndInstall((event) => {
		if (event.event === 'Started') total = event.data.contentLength ?? 0;
		if (event.event === 'Progress' && total) {
			done += event.data.chunkLength;
			progress(Math.round((done / total) * 100));
		}
	});
}
