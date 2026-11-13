import { describe, expect, it } from 'vitest';
import { hasSettings, resolveSettings, visibleSettings } from './settings';
import type { WidgetSetting } from './types';

const options = (...values: string[]) => values.map((value) => ({ value, label: value }));

const schema: WidgetSetting[] = [
	{
		key: 'metric',
		label: 'Metric',
		type: 'select',
		default: 'cpu',
		options: options('cpu', 'net')
	},
	{
		key: 'direction',
		label: 'Direction',
		type: 'select',
		options: options('down', 'up'),
		when: { metric: 'net' }
	}
];

describe('resolveSettings', () => {
	it('fills unset keys with the default, else the first option', () => {
		expect(resolveSettings(schema)).toEqual({ metric: 'cpu', direction: 'down' });
	});

	it('keeps saved values', () => {
		expect(resolveSettings(schema, { metric: 'net', direction: 'up' })).toEqual({
			metric: 'net',
			direction: 'up'
		});
	});

	it('drops values that are not an option, and keys that are not declared', () => {
		expect(resolveSettings(schema, { metric: 'gpu', other: 'x' })).toEqual({
			metric: 'cpu',
			direction: 'down'
		});
	});

	it('keeps any saved value of a select with a source, which only the settings list', () => {
		const sourced: WidgetSetting[] = [
			{
				key: 'channel',
				label: 'Channel',
				type: 'select',
				options: options('all'),
				source: 'audioApps'
			}
		];
		expect(resolveSettings(sourced, { channel: 'Discord.exe' })).toEqual({
			channel: 'Discord.exe'
		});
		expect(resolveSettings(sourced, { channel: '' })).toEqual({ channel: 'all' });
	});

	it('keeps any saved text, else its default, else empty', () => {
		const text: WidgetSetting[] = [
			{ key: 'url', label: 'URL', type: 'text' },
			{ key: 'title', label: 'Title', type: 'text', default: 'Home' }
		];
		expect(resolveSettings(text)).toEqual({ url: '', title: 'Home' });
		expect(resolveSettings(text, { url: 'https://a.b', title: '' })).toEqual({
			url: 'https://a.b',
			title: ''
		});
	});
});

it('visibleSettings hides settings whose condition does not hold', () => {
	expect(visibleSettings(schema, { metric: 'cpu' }).map((s) => s.key)).toEqual(['metric']);
	expect(visibleSettings(schema, { metric: 'net' }).map((s) => s.key)).toEqual([
		'metric',
		'direction'
	]);
});

it('visibleSettings shows a setting while its value is one of a list', () => {
	const listed: WidgetSetting[] = [
		{ key: 'metric', label: 'Metric', type: 'select', options: options('cpu', 'gpu', 'net') },
		{
			key: 'direction',
			label: 'Direction',
			type: 'select',
			options: options('down', 'up'),
			when: { metric: ['cpu', 'gpu'] }
		}
	];
	const keys = (metric: string) => visibleSettings(listed, { metric }).map((s) => s.key);
	expect(keys('gpu')).toEqual(['metric', 'direction']);
	expect(keys('net')).toEqual(['metric']);
});

it('visibleSettings hides a setting while its unless condition holds', () => {
	const shown = (unless: WidgetSetting['unless'], metric: string) =>
		visibleSettings(
			[{ key: 'direction', label: 'Direction', type: 'select', options: options('up'), unless }],
			{ metric }
		).length > 0;
	expect(shown({ metric: 'net' }, 'net')).toBe(false);
	expect(shown({ metric: 'net' }, 'cpu')).toBe(true);
	expect(shown({ metric: ['cpu', 'net'] }, 'cpu')).toBe(false);
	expect(shown({}, 'cpu')).toBe(true);
});

describe('hasSettings', () => {
	it('is false for no widget, or one without settings', () => {
		expect(hasSettings(undefined)).toBe(false);
		expect(hasSettings({})).toBe(false);
		expect(hasSettings({ settings: [] })).toBe(false);
	});

	it('is true when a setting shows', () => {
		expect(hasSettings({ settings: schema })).toBe(true);
	});

	it('is false when every setting is hidden by its condition', () => {
		expect(hasSettings({ settings: schema.slice(1) }, { metric: 'cpu' })).toBe(false);
	});

	it('is true for a webview widget, which can pick a profile', () => {
		expect(hasSettings({ permissions: ['webview'] })).toBe(true);
	});
});
