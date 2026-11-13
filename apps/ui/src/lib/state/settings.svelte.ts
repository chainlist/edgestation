import { DEFAULT_PAGES } from '#lib/layout/defaults.ts';
import type { Page } from '#lib/layout/types.ts';
import type { ShortcutAction, ShortcutChanges } from '#lib/shortcuts/actions.ts';
import { DEFAULT_THEME } from '#lib/theme/presets.ts';
import type { ThemeSettings } from '#lib/theme/types.ts';
import type { ListingSort } from '#lib/widgets/marketplace/listing.ts';
import { persisted } from './persisted.svelte';

export interface DisplaySettings {
	/**
	 * The name of the monitor for the Edge window. With `null`, the Edge uses the first monitor
	 * that is not the main screen.
	 */
	monitor: string | null;
}

/** The direction of the Edge pages, for the layout and for a swipe. */
export type PageDirection = 'horizontal' | 'vertical';

/**
 * How the Edge goes from one page to the next. `slide` moves the pages along the swipe axis.
 * `fade` blends one page into the other in place.
 */
export type PageTransition = 'slide' | 'fade';

/**
 * A browser profile for widget web views, with its own sign-ins and cookies. The list does not
 * include the Default profile, wich is the browser data of the app.
 */
export interface Profile {
	/** Also the name of its data folder, so use only letters, digits and dashes. */
	id: string;
	name: string;
}

export const theme = persisted<ThemeSettings>('theme', DEFAULT_THEME);
/**
 * The image behind the Edge pages as a data URL (see `readBackground`), or `null`. It has its own
 * key, so a change of blur does not send the image to all windows again.
 */
export const backgroundImage = persisted<string | null>('backgroundImage', null);
/** The blur of the background image, from 0 (sharp) to 1. */
export const backgroundBlur = persisted<number>('backgroundBlur', 0.15);
/** The opacity of the widget cards on the Edge, from 0 to 1 (opaque). */
export const widgetOpacity = persisted<number>('widgetOpacity', 0.5);
/** The blur of what shows through the widget cards on the Edge, from 0 (sharp) to 1. */
export const widgetBlur = persisted<number>('widgetBlur', 0.7);
export const pages = persisted<Page[]>('pages', DEFAULT_PAGES);
/** The pages that show widget `id`, in page order. */
export const pagesShowing = (id: string) =>
	pages.current.filter((page) => page.items.some((item) => item.widget === id));
export const pageDirection = persisted<PageDirection>('pageDirection', 'horizontal');
export const pageTransition = persisted<PageTransition>('pageTransition', 'slide');
export const display = persisted<DisplaySettings>('display', { monitor: null });
/** If true, the widgets on the Edge pause while another app is full screen. */
export const freezeOnFullscreen = persisted<boolean>('freezeOnFullscreen', true);
export const profiles = persisted<Profile[]>('profiles', []);

/** The shortcuts that the user changed or turned off. The other shortcuts keep the the default. */
export const shortcuts = persisted<ShortcutChanges>('shortcuts', {});
/**
 * The shortcuts that the Edge window could not register, usually because another app has them.
 * The Edge window writes this list, and the settings window shows it.
 */
export const shortcutsFailed = persisted<ShortcutAction[]>('shortcutsFailed', []);

/** If true, install newer releases of marketplace widgets at launch. The host also reads it. */
export const widgetAutoUpdate = persisted<boolean>('widgetAutoUpdate', true);
/** The order of Settings > Widgets > Browse. */
export const browseSort = persisted<ListingSort>('browseSort', 'downloads');

export const settingsReady = Promise.all([
	theme.ready,
	backgroundImage.ready,
	backgroundBlur.ready,
	widgetOpacity.ready,
	widgetBlur.ready,
	pages.ready,
	pageDirection.ready,
	pageTransition.ready,
	display.ready,
	freezeOnFullscreen.ready,
	profiles.ready,
	shortcuts.ready,
	shortcutsFailed.ready
]);
