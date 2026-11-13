import type { Placement } from '#lib/layout/types.ts';

/**
 * The widgets that the user added to a page but that do not fit on it yet, by page id. The page
 * editor shows them in red. They are not saved, so the Edge does not show them. They stay while
 * the settings window is open. Change them with `editWaiting`.
 */
export const waiting = $state<Record<string, Placement[]>>({});
