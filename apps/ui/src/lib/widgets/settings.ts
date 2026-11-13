import { usesWebview } from './permissions';
import type { SettingCondition, WidgetManifest, WidgetSetting, WidgetSettingValues } from './types';

/**
 * Returns the values that a widget sees. Each declared setting gets its saved value, else its
 * default. A saved select value that is not an option now also falls back, for example after a
 * widget update. A select with a `source` keeps a saved value, because its choices are known
 * only when the settings show.
 */
export function resolveSettings(
	schema: WidgetSetting[] = [],
	saved: WidgetSettingValues = {}
): WidgetSettingValues {
	const values: WidgetSettingValues = {};
	for (const setting of schema) {
		if (setting.type === 'text') {
			values[setting.key] = saved[setting.key] ?? setting.default ?? '';
			continue;
		}
		const valid = (value?: string) =>
			setting.source ? !!value : setting.options.some((option) => option.value === value);
		const fallback = valid(setting.default) ? setting.default : setting.options[0]?.value;
		const value = valid(saved[setting.key]) ? saved[setting.key] : fallback;
		if (value !== undefined) values[setting.key] = value;
	}
	return values;
}

const holds = (condition: SettingCondition, values: WidgetSettingValues) =>
	Object.entries(condition).every(([key, value]) => [value].flat().includes(values[key]));

export const visibleSettings = (schema: WidgetSetting[], values: WidgetSettingValues) =>
	schema.filter(
		// An empty condition always holds. So an empty `unless` must hide nothing, not everything.
		({ when = {}, unless = {} }) =>
			holds(when, values) && (Object.keys(unless).length === 0 || !holds(unless, values))
	);

/**
 * Tells if a placement of `widget` has something to set. This is a profile for webview widgets,
 * or a setting that shows for its `saved` values. The page editor opens no settings for others.
 */
export const hasSettings = (
	widget: Pick<WidgetManifest, 'settings' | 'permissions'> | undefined,
	saved?: WidgetSettingValues
) =>
	!!widget &&
	(usesWebview(widget) ||
		visibleSettings(widget.settings ?? [], resolveSettings(widget.settings, saved)).length > 0);
