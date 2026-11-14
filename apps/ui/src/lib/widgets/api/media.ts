import { invoke } from '@tauri-apps/api/core';
import type { Scope } from '../scope';
import type { Cleanup, MediaAction, MediaApi, MediaInfo } from '../types';
import { stream, watchChanges } from './changes';
import type { Pause } from './timers';

const readers = Object.freeze({
	current: () => invoke<MediaInfo | null>('media_info'),
	sessions: () => invoke<MediaInfo[]>('media_sessions'),
	art: (id?: string) => invoke<string | null>('media_art', { id }),
	control: (action: MediaAction, id?: string) => invoke<boolean>('media_control', { action, id }),
	seek: (position: number, id?: string) => invoke<boolean>('media_seek', { position, id })
});

/** The host sends this when a session opens, closes or changes, while a widget listens. */
const changes = stream<null>('media:changed', 'media_watch');

/**
 * The `api.media` of one widget. Its listeners listen only while its page shows and the Edge is
 * not frozen (`pause`). `guard` sends their errors to its error card. `track` stops them when the
 * widget unmounts.
 */
export const createMediaApi = (
	pause: Pause,
	guard: Scope['guard'],
	track: (stop: Cleanup) => Cleanup
): MediaApi => Object.freeze({ ...readers, onChange: watchChanges(changes, pause, guard, track) });
