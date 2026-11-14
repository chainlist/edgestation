import { vi } from 'vitest';

export const coreModule = () => ({ invoke: vi.fn(async () => []) });
export const openWebviewModule = () => ({ openWebview: vi.fn(() => () => {}) });
export const httpModule = () => ({ fetch: vi.fn() });
export const scrollbarsModule = () => ({ overlayScrollbars: vi.fn() });
