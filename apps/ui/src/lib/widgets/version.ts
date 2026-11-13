function parse(version: string): number[] | undefined {
	const match = /^(\d+)\.(\d+)\.(\d+)/.exec(version.trim());
	return match ? match.slice(1).map(Number) : undefined;
}

/**
 * Tells if `candidate` is a later `major.minor.patch` than `current`. A `candidate` that does
 * not parse is never newer. A `current` that does not parse is older than all versions. The
 * marketplace updater in the host compares the same way.
 */
export function isNewer(candidate: string, current: string): boolean {
	const next = parse(candidate);
	if (!next) return false;
	const have = parse(current) ?? [0, 0, 0];
	for (let i = 0; i < 3; i++) {
		if (next[i] !== have[i]) return next[i] > have[i];
	}
	return false;
}

/**
 * Tells why an app at `appVersion` cannot run a widget that needs `minAppVersion`. Returns
 * `undefined` when it can. Only `major.minor.patch` counts, so a pre-release of the app
 * (`0.2.0-beta`) runs widgets that need it's release (`0.2.0`).
 */
export function versionProblem(appVersion: string, minAppVersion: string): string | undefined {
	const need = parse(minAppVersion);
	if (!need) return `its minAppVersion "${minAppVersion}" is not a version like 0.1.0`;
	const have = parse(appVersion) ?? [0, 0, 0];
	for (let i = 0; i < 3; i++) {
		if (have[i] > need[i]) return undefined;
		if (have[i] < need[i]) {
			return `it needs EdgeStation ${minAppVersion} or later, and this is ${appVersion}`;
		}
	}
	return undefined;
}
