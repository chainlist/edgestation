import { REPO_URL, VERSION } from './site.ts';

/** A file that the release workflow (desktop-release.yml) attaches to each release. */
export type Installer = {
	/** The name of the file in the release. */
	file: string;
	/** A direct link to the file, so that the browser downloads it at once. */
	url: string;
};

/**
 * The links name the tag of the release of VERSION, not the latest release, because each file name
 * contains the version. So a link of the site stays correct after a newer release comes out, until
 * release-please moves VERSION to it.
 */
function installer(file: string): Installer {
	return { file, url: `${REPO_URL}/releases/download/v${VERSION}/${file}` };
}

export const windows = {
	setup: installer(`EdgeStation_${VERSION}_x64-setup.exe`),
	msi: installer(`EdgeStation_${VERSION}_x64_en-US.msi`)
};

export const linux = {
	deb: installer(`EdgeStation_${VERSION}_amd64.deb`),
	rpm: installer(`EdgeStation-${VERSION}-1.x86_64.rpm`),
	appImage: installer(`EdgeStation_${VERSION}_amd64.AppImage`)
};

export type Platform = 'windows' | 'linux';

/**
 * The platform of the visitor, from the user agent of the browser. It is undefined on a system that
 * EdgeStation does not run on, such as macOS or a phone. Android and ChromeOS also say Linux, so
 * they are checked first.
 */
export function detectPlatform(userAgent: string): Platform | undefined {
	if (/Windows/i.test(userAgent)) return 'windows';
	if (/Android|CrOS/i.test(userAgent)) return undefined;
	if (/Linux/i.test(userAgent)) return 'linux';
	return undefined;
}
