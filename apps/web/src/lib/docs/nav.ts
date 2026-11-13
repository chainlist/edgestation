/** One page of the docs. Its content is in `pages/<slug>.md`, or `index.md` for `/docs`. */
export interface DocPage {
	slug: string;
	title: string;
	/** One sentence under the title. It is also the meta description of the page. */
	description: string;
}

export interface DocSection {
	title: string;
	pages: DocPage[];
}

/** The sidebar, in reading order. Previous and Next also use this order. */
export const sections: DocSection[] = [
	{
		title: 'Getting started',
		pages: [
			{
				slug: '',
				title: 'Introduction',
				description: 'What a widget is, where EdgeStation finds it, and the ways to write one.'
			},
			{
				slug: 'getting-started',
				title: 'Quick start',
				description:
					'Write a widget by hand, then the same one with the SDK, and see it on the Edge.'
			},
			{
				slug: 'dependencies',
				title: 'Installing the SDK',
				description:
					'What to install for plain JavaScript, Svelte, React or Vue, and how npm packages end up in your widget.'
			}
		]
	},
	{
		title: 'Guides',
		pages: [
			{
				slug: 'manifest',
				title: 'The manifest',
				description: 'Every field of manifest.json, and the schema that checks it in your editor.'
			},
			{
				slug: 'settings',
				title: 'Settings',
				description:
					'Options the user sets for each placement of the widget, read from api.settings.'
			},
			{
				slug: 'frameworks',
				title: 'Svelte, React and Vue',
				description: 'Mount a component as a widget with the SDK adapters.'
			},
			{
				slug: 'styling',
				title: 'Styling and theming',
				description: 'Lay the widget out in its cell and follow the colors the user picked.'
			},
			{
				slug: 'lifecycle',
				title: 'Lifecycle and errors',
				description:
					'Mounting, pausing while off screen or frozen, and how errors stay in their tile.'
			},
			{
				slug: 'build',
				title: 'Build, test and share',
				description:
					'Build with the Vite plugin, load your widget in EdgeStation while you work, and ship it.'
			}
		]
	},
	{
		title: 'API reference',
		pages: [
			{
				slug: 'api',
				title: 'The api object',
				description: 'Everything a widget receives when it mounts, at a glance.'
			},
			{
				slug: 'api/system',
				title: 'System and programs',
				description:
					'CPU, GPU, network and Wi-Fi with api.system(), and the busiest programs with api.topPrograms().'
			},
			{
				slug: 'api/media',
				title: 'Media',
				description: 'What is playing, its album art and its playback controls, with api.media.'
			},
			{
				slug: 'api/audio',
				title: 'Audio',
				description:
					'Volumes, mute and level meters of the output, the microphone and each app, with api.audio.'
			},
			{
				slug: 'api/apps',
				title: 'Apps',
				description:
					'List, start and draw the apps of the Start menu or the app menu with api.apps.'
			},
			{
				slug: 'api/fetch',
				title: 'Fetch',
				description: 'Call any HTTP API from a widget with api.fetch, without CORS in the way.'
			},
			{
				slug: 'api/storage',
				title: 'Storage',
				description: 'Keep data across remounts and restarts, per placement or for every copy.'
			},
			{
				slug: 'api/webview',
				title: 'Web views',
				description: 'Show a whole web page in a native browser view with api.webview().'
			},
			{
				slug: 'api/helpers',
				title: 'Timers and helpers',
				description:
					'api.every, api.after, api.guard, api.loading, api.onDispose, api.asset and api.scrollbars.'
			},
			{
				slug: 'sdk',
				title: 'SDK reference',
				description: 'What each entry point of @edgestation/sdk exports.'
			}
		]
	}
];

export const pages: DocPage[] = sections.flatMap((section) => section.pages);
