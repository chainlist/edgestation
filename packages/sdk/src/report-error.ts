import type { WidgetApi } from './types.js';

/**
 * Shows `error` on the error card of the widget. The API has no direct call for this, so a
 * guarded callback throws the error.
 */
export function report(api: WidgetApi, error: unknown): void {
	api.guard(() => {
		throw error;
	})();
}
