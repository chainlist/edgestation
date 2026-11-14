import { afterEach, beforeEach, vi } from 'vitest';
import { createWidgetApi } from './widget-api';
import { createScope } from '../scope';
import { createPause } from './timers';
import type { WidgetPermission } from '../types';
import { openWebview } from '../webview/open-webview';

const ALL: WidgetPermission[] = ['system', 'media', 'apps', 'audio', 'fetch', 'webview'];

/** `permissions` is the list in the manifest. */
export const setup = ({
	permissions = ALL,
	profile
}: { permissions?: WidgetPermission[]; profile?: string } = {}) => {
	const fail = vi.fn();
	const scope = createScope(fail);
	const pause = createPause(false);
	const api = createWidgetApi(
		'instance',
		{ id: 'test', name: 'Test', permissions },
		{},
		profile,
		scope,
		pause,
		vi.fn()
	);
	return { api, scope, pause, fail };
};

beforeEach(() => {
	vi.useFakeTimers();
	vi.mocked(openWebview).mockClear();
});
afterEach(() => vi.useRealTimers());
