import { invoke } from '@tauri-apps/api/core';

/** Each call waits for the previous call, so the release cannot come before the boxes exist. */
let queue: Promise<unknown> = Promise.resolve();
const run = (command: string) => (queue = queue.then(() => invoke(command)));

/**
 * Makes the identify boxes in advance and keeps them hidden, so `identifyScreens` can show them
 * at once. An new web view needs about half a second to start. Returns the function that removes
 * the boxes.
 */
export function prepareIdentify(): () => void {
	run('identify_prepare').catch(console.error);
	return () => void run('identify_release').catch(console.error);
}

/** Shows the name of each monitor in the middle of it, with the name from the display settings. */
export const identifyScreens = () => run('identify_screens');
