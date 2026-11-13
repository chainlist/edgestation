/** The direction in which the settings content moves on a navigation. */
export type SettingsTransition = 'forward' | 'back' | 'up' | 'down';

/**
 * Between sections, the content moves in the sidebar direction: `down` to a lower section, `up`
 * to a higher one. `sections` is in sidebar order, and a path matches a section by its href
 * prefix. In a section, a sub-page with more path segments opens `forward` and closes `back`.
 */
export function settingsTransition(
	from: string,
	to: string,
	sections: readonly { href: string }[]
): SettingsTransition | null {
	const section = (path: string) => sections.findIndex(({ href }) => path.startsWith(href));
	const depth = (path: string) => path.split('/').length;
	const a = section(from);
	const b = section(to);
	if (a !== b) return b > a ? 'down' : 'up';
	if (depth(from) === depth(to)) return null;
	return depth(to) > depth(from) ? 'forward' : 'back';
}
