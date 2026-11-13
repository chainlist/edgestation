import type { AudioTarget } from '@edgestation/sdk';
import type { Icon } from './icons.ts';

/** Icon sizes to ask the host for. All strips use these few sizes, so they share images. */
const ICON_SIZES = [32, 48, 64, 96, 128];
/** About the shown size of the icons, in screen pixels. */
export const ICON_PX =
	ICON_SIZES.find((size) => size >= 64 * (window.devicePixelRatio || 1)) ?? 128;

/** One fader, with what it sets and what it shows. */
export interface Strip {
	target: AudioTarget;
	name: string;
	volume: number;
	muted: boolean;
	/** The URL of the icon, or `null` to use `fallback`. */
	src: string | null;
	fallback: Icon;
	/** Nothing to set. The app of the channel is not open, or its device is not their. */
	off?: boolean;
	/** A stream is open on it, so it can make sound. Its meter reads levels only then. */
	live?: boolean;
}
