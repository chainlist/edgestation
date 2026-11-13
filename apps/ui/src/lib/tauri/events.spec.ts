import { emitTo, type EventCallback } from '@tauri-apps/api/event';
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
import { expect, it, vi } from 'vitest';
import { onShowPageRequest, showPageOnEdge } from './events';

vi.mock('@tauri-apps/api/event', () => ({ emitTo: vi.fn(async () => {}), listen: vi.fn() }));
vi.mock('@tauri-apps/api/webviewWindow', () => {
	const listen = vi.fn(async () => () => {});
	return { getCurrentWebviewWindow: () => ({ listen }) };
});

it('turns the Edge to the page that Settings asks for', async () => {
	const fn = vi.fn();
	await onShowPageRequest(fn);
	await showPageOnEdge('p2');

	const [target, sent, payload] = vi.mocked(emitTo).mock.calls[0];
	expect(target).toBe('edge');
	const [heard, handler] = vi.mocked(getCurrentWebviewWindow().listen).mock.calls[0];
	expect(heard).toBe(sent);

	(handler as EventCallback<unknown>)({ event: sent, id: 1, payload });
	expect(fn).toHaveBeenCalledWith('p2');
});
