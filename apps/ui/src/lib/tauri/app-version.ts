import { getVersion } from '@tauri-apps/api/app';

let version: Promise<string> | undefined;

/** The version of EdgeStation. A window asks the host once, and again only after a failure. */
export function appVersion() {
	version ??= getVersion().catch((error: unknown) => {
		version = undefined;
		throw error;
	});
	return version;
}
