import type { WidgetManifest } from '@edgestation/sdk';

// The SDK that widget authors install defines what widgets see: their `api`, manifest and
// module. So the host and the widgets cannot drift apart.
export type * from '@edgestation/sdk';

/** A widget as the host lists it in the settings window. */
export interface InstalledWidget extends WidgetManifest {
	/** Ships with the app, comes from the marketplace, or the user put it in the widgets folder. */
	source: 'builtin' | 'marketplace' | 'local';
	/** Set when the host could not read `manifest.json`. */
	error?: string;
}

/** A widget of the marketplace, as the host lists it (`marketplace_list`). */
export interface ListedWidget {
	id: string;
	/** Its GitHub repo, `owner/name`. */
	repo: string;
	/** The download count of its `widget.zip` for all releases. `null` untill the stats have it. */
	downloads: number | null;
	/** The publish date of its latest release. `null` until the stats have it. */
	released: string | null;
	/** The manifest of its latest release. `null` with `error` if the host could not read it. */
	manifest: Omit<WidgetManifest, 'id'> | null;
	error: string | null;
	/** URLs of its icon and README on the default branch of the repo. */
	icon: string | null;
	readme: string;
}
