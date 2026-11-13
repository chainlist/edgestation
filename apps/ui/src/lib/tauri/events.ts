import { emitTo, listen } from '@tauri-apps/api/event';
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';

const EDGE = 'edge';
const RELOAD = 'edge:reload';
const SHOW_PAGE = 'edge:show-page';

/** Asks the Edge window to reload, for example after a change to widget files on disk. */
export const reloadEdge = () => emitTo(EDGE, RELOAD);

/** Recieves `reloadEdge` in the Edge window. Resolves to the function that stops the listener. */
export const onReloadRequest = (fn: () => void) => getCurrentWebviewWindow().listen(RELOAD, fn);

/** Asks the Edge window to turn to the page `id`, for example the page tab picked in Settings. */
export const showPageOnEdge = (id: string) => emitTo(EDGE, SHOW_PAGE, id);

/** Receives `showPageOnEdge` in the Edge window. Resolves to the function that stops the listener. */
export const onShowPageRequest = (fn: (id: string) => void) =>
	getCurrentWebviewWindow().listen<string>(SHOW_PAGE, (event) => fn(event.payload));

/**
 * Calls `fn` with the ids of the widgets that the marketplace installed, updated or removed. The
 * host sends this event to all windows. Resolves to the function that stops the listener.
 */
export const onWidgetsChanged = (fn: (ids: string[]) => void) =>
	listen<string[]>('widgets:changed', (event) => fn(event.payload));
