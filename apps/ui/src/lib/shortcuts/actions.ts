import type { Shortcut } from './keys';

/**
 * All the actions that a shortcut can do. Settings → Shortcuts shows each one under its group.
 * To add a shortcut, add it here and make its action in the Edge window.
 */
export const SHORTCUT_ACTIONS = {
	previousPage: {
		group: 'Pages',
		label: 'Previous page',
		description: 'Go back one page on the Edge.',
		default: 'Control+Alt+PageUp'
	},
	nextPage: {
		group: 'Pages',
		label: 'Next page',
		description: 'Go forward one page on the Edge.',
		default: 'Control+Alt+PageDown'
	},
	goToPage: {
		group: 'Pages',
		label: 'Go to page',
		description: "Type part of a page's name on the Edge to go straight to it.",
		default: 'Control+Alt+KeyG'
	},
	bringToFront: {
		group: 'Window',
		label: 'Bring to front',
		description: 'Put the Edge back above a window that covers it.',
		default: 'Control+Alt+Home'
	},
	toggleEdge: {
		group: 'Window',
		label: 'Hide or show',
		description: 'Hide the Edge, or show it again. Its widgets pause while it is hidden.',
		default: 'Control+Alt+KeyT'
	}
} satisfies Record<
	string,
	{ group: string; label: string; description: string; default: Shortcut | null }
>;

export type ShortcutAction = keyof typeof SHORTCUT_ACTIONS;

/** The actions in settings order. */
export const ACTIONS = Object.keys(SHORTCUT_ACTIONS) as ShortcutAction[];

/** The shortcut of each action, or `null` if the shortcut is off. */
export type Shortcuts = Record<ShortcutAction, Shortcut | null>;

/**
 * The changes of the user: a custom shortcut, or `null` for a shortcut that is off. An action
 * that is not in it keeps its default, so a new default gets to all users who did not chaneg it.
 */
export type ShortcutChanges = Partial<Shortcuts>;

export function resolveShortcuts(changes: ShortcutChanges): Shortcuts {
	return Object.fromEntries(
		ACTIONS.map((action) => {
			const changed = changes[action];
			return [action, changed === undefined ? SHORTCUT_ACTIONS[action].default : changed];
		})
	) as Shortcuts;
}

/** Sets the shortcut of `action`. If the shortcut is the default again, the change goes away. */
export function changeShortcut(
	changes: ShortcutChanges,
	action: ShortcutAction,
	shortcut: Shortcut | null
): ShortcutChanges {
	const { [action]: _, ...others } = changes;
	return shortcut === SHORTCUT_ACTIONS[action].default ? others : { ...others, [action]: shortcut };
}

/** Finds the other action that has `shortcut`, because one shortcut cannot do two things. */
export function usedBy(
	shortcuts: Shortcuts,
	shortcut: Shortcut,
	action: ShortcutAction
): ShortcutAction | undefined {
	return ACTIONS.find((other) => other !== action && shortcuts[other] === shortcut);
}
