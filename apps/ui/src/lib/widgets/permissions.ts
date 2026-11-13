import type { InstalledWidget, WidgetManifest, WidgetPermission } from './types';

type Declares = Pick<WidgetManifest, 'permissions'>;

/**
 * What each permission lets a widget do, in display order. `label` names it on a widget row, and
 * `allows` is a line of the install dialog. `tried` completes "It tried to ..." on the error card.
 */
export const PERMISSIONS: Record<
	WidgetPermission,
	{ label: string; allows: string; tried: string }
> = {
	system: {
		label: 'PC use',
		allows: 'See your CPU, GPU and network use, and the busiest programs.',
		tried: 'read how busy your PC is'
	},
	media: {
		label: 'Media',
		allows: 'See what is playing, and play, pause or skip it.',
		tried: 'control what is playing'
	},
	apps: {
		label: 'Apps',
		allows: 'See the apps of your Start menu or app menu and open them.',
		tried: 'reach your apps'
	},
	audio: {
		label: 'Sound',
		allows: 'See and change the volume of your speakers, microphone and apps.',
		tried: 'reach your sound settings'
	},
	fetch: {
		label: 'Internet',
		allows: 'Connect to any website or online service.',
		tried: 'connect to the internet'
	},
	webview: {
		label: 'Web pages',
		allows: 'Open web pages with the sign-ins of a browser profile.',
		tried: 'open a web page'
	}
};

/**
 * The permissions that `widget` asks for, in display order. The manifest is not checked at run
 * time, so this drops repeats and unkown names.
 */
export const declaredPermissions = (widget: Declares): WidgetPermission[] =>
	(Object.keys(PERMISSIONS) as WidgetPermission[]).filter((permission) =>
		widget.permissions?.includes(permission)
	);

/** Built-in widgets ship with the app, so only the others show what they ask for. */
export const shownPermissions = (widget: InstalledWidget) =>
	widget.source === 'builtin' ? [] : declaredPermissions(widget);

/** The permissions that `latest` asks for and `installed` does not have, in display order. */
export const addedPermissions = (installed: Declares, latest: Declares) =>
	declaredPermissions(latest).filter((permission) => !installed.permissions?.includes(permission));

/** Tells if `widget` can call `api.webview`. Its placements then also get a Profile setting. */
export const usesWebview = (widget: Declares) => !!widget.permissions?.includes('webview');
