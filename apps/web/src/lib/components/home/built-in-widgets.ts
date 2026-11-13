import Clock from '@lucide/svelte/icons/clock';
import CloudSun from '@lucide/svelte/icons/cloud-sun';
import EighthNotes from '@lucide/svelte/icons/eighth-notes';
import Gauge from '@lucide/svelte/icons/gauge';
import Globe from '@lucide/svelte/icons/globe';
import ListChecks from '@lucide/svelte/icons/list-checks';
import ListStart from '@lucide/svelte/icons/list-start';
import Rocket from '@lucide/svelte/icons/rocket';
import SlidersVertical from '@lucide/svelte/icons/sliders-vertical';
import TvMinimalPlay from '@lucide/svelte/icons/tv-minimal-play';

export const builtInWidgets = [
	{
		icon: Clock,
		name: 'Clock',
		text: 'The time and date, in 12 or 24 hours and the date format you like.'
	},
	{
		icon: CloudSun,
		name: 'Weather',
		text: 'The next 24 hours as a temperature curve over the chance of rain, the temperature now, or today in detail.'
	},
	{
		icon: Gauge,
		name: 'Performance',
		text: 'CPU or GPU load, GPU temperature, internet usage with averages, the top programs.'
	},
	{
		icon: EighthNotes,
		name: 'Media',
		text: "What's playing (Spotify, YouTube, ...) with album art, progress and controls."
	},
	{
		icon: SlidersVertical,
		name: 'Mixer',
		text: 'Volume faders with level meters, or mute buttons only: every channel, or one per placement.'
	},
	{
		icon: Rocket,
		name: 'Launcher',
		text: 'Keys that open apps from the Start menu or the app menu, like a Stream Deck, on your main screen.'
	},
	{
		icon: ListChecks,
		name: 'To-do',
		text: 'A to-do list: type a task, tap it to check it off, clear the done ones.'
	},
	{
		icon: ListStart,
		name: 'Logs timeline',
		text: 'Type what you are doing and it joins a timeline of your day. Browse earlier days.'
	},
	{
		icon: Globe,
		name: 'URL',
		text: 'Any web page, in a real browser view. Sign-ins last across restarts.'
	},
	{
		icon: TvMinimalPlay,
		name: 'YouTube',
		text: 'YouTube, opening on your Subscriptions. Sign in once inside it.'
	}
];
