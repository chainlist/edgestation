// @vitest-environment node
import { Ajv } from 'ajv';
import { existsSync, readdirSync, readFileSync } from 'node:fs';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { describe, expect, it } from 'vitest';

const readJson = (path: string) => JSON.parse(readFileSync(path, 'utf8'));
const schema = readJson(fileURLToPath(new URL('../manifest.schema.json', import.meta.url)));
const validate = new Ajv({ allErrors: true }).compile(schema);

const widgets = fileURLToPath(new URL('../../../widgets/', import.meta.url));
const builtins = readdirSync(widgets).filter((id) =>
	existsSync(join(widgets, id, 'manifest.json'))
);

const select = {
	key: 'metric',
	label: 'Metric',
	type: 'select',
	options: [{ value: 'cpu', label: 'CPU' }]
};

describe('manifest.schema.json', () => {
	it('finds the built-in widgets', () => {
		expect(builtins).toContain('clock');
	});

	it.each(builtins)('accepts the manifest of the built-in %s widget', (id) => {
		const valid = validate(readJson(join(widgets, id, 'manifest.json')));
		expect(validate.errors).toBeNull();
		expect(valid).toBe(true);
	});

	it('accepts every documented field', () => {
		const manifest = {
			$schema: './node_modules/@edgestation/sdk/manifest.schema.json',
			name: 'My widget',
			version: '1.0.0',
			description: 'What it shows.',
			author: 'You',
			minAppVersion: '0.1.0',
			main: 'dist/main.js',
			styles: 'dist/main.css',
			icon: 'icon.svg',
			size: { w: 2, h: 1 },
			permissions: ['system', 'media', 'apps', 'audio', 'fetch', 'webview'],
			settings: [
				{ ...select, default: 'cpu' },
				{
					key: 'url',
					label: 'URL',
					type: 'text',
					placeholder: 'https://',
					when: { metric: 'cpu' }
				},
				{ key: 'token', label: 'Token', type: 'text', secret: true }
			]
		};
		expect(validate(manifest)).toBe(true);
	});

	it.each([
		['no name', { version: '1.0.0' }],
		['a misspelled field', { name: 'W', setting: [] }],
		['a size with no height', { name: 'W', size: { w: 2 } }],
		['a minAppVersion that is not x.y.z', { name: 'W', minAppVersion: '>=0.1' }],
		['a select setting with no options', { name: 'W', settings: [{ ...select, options: [] }] }],
		['a setting of an unknown type', { name: 'W', settings: [{ ...select, type: 'number' }] }],
		['a text setting with options', { name: 'W', settings: [{ ...select, type: 'text' }] }],
		['an unknown permission', { name: 'W', permissions: ['camera'] }],
		['a permission twice', { name: 'W', permissions: ['media', 'media'] }],
		['the old webview flag', { name: 'W', webview: true }]
	])('rejects a manifest with %s', (_, manifest) => {
		expect(validate(manifest)).toBe(false);
	});
});
