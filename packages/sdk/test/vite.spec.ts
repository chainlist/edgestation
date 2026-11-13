// @vitest-environment node
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, join } from 'node:path';
import { pathToFileURL } from 'node:url';
import { build } from 'vite';
import { afterEach, describe, expect, it } from 'vitest';
import { widget } from '../src/vite.js';

const projects: string[] = [];

afterEach(() => {
	for (const root of projects.splice(0)) rmSync(root, { recursive: true, force: true });
});

/** Makes a widget project in a temporay folder from the file contents, by path. */
function project(files: Record<string, string>): string {
	const root = mkdtempSync(join(tmpdir(), 'edgestation-widget-'));
	projects.push(root);
	for (const [path, text] of Object.entries(files)) {
		mkdirSync(dirname(join(root, path)), { recursive: true });
		writeFileSync(join(root, path), text);
	}
	return root;
}

/** Builds the project with the preset. Resolves to the warnings that it printed. */
async function buildWidget(root: string): Promise<string[]> {
	const warnings: string[] = [];
	await build({
		root,
		configFile: false,
		logLevel: 'silent',
		plugins: [widget()],
		build: { rollupOptions: { onwarn: (warning) => void warnings.push(warning.message) } }
	});
	return warnings;
}

const entry = 'export default { mount(el) { el.textContent = process.env.NODE_ENV; } };';
const manifest = (fields: object) => JSON.stringify({ name: 'Test', ...fields });

describe('widget()', () => {
	it('builds src/main.js into a self-contained dist/main.js', async () => {
		const root = project({
			'manifest.json': manifest({ main: 'dist/main.js' }),
			'src/main.js': `import { label } from './label.js';\n${entry.replace('process', 'label + process')}`,
			'src/label.js': 'export const label = "env: ";'
		});
		const warnings = await buildWidget(root);

		const file = join(root, 'dist/main.js');
		expect(readFileSync(file, 'utf8')).not.toContain('process.env');
		const { default: module } = await import(pathToFileURL(file).href);
		const el = { textContent: '' };
		module.mount(el);
		expect(el.textContent).toBe('env: production');
		expect(warnings).toEqual([]);
	});

	it('finds a TypeScript entry', async () => {
		const root = project({
			'manifest.json': manifest({ main: 'dist/main.js' }),
			'src/main.ts': `const mode: string = process.env.NODE_ENV!;\n${entry.replace('process.env.NODE_ENV', 'mode')}`
		});
		await buildWidget(root);
		expect(existsSync(join(root, 'dist/main.js'))).toBe(true);
	});

	it('warns when manifest.json does not point at the built module', async () => {
		const root = project({ 'manifest.json': manifest({}), 'src/main.js': entry });
		const warnings = await buildWidget(root);
		expect(warnings.join('\n')).toContain('set "main": "dist/main.js"');
	});

	it('writes imported CSS to dist/main.css, and warns until manifest.json loads it', async () => {
		const files = {
			'src/main.js': `import './style.css';\n${entry}`,
			'src/style.css': 'p { color: red; }'
		};
		const missing = project({ ...files, 'manifest.json': manifest({ main: 'dist/main.js' }) });
		expect((await buildWidget(missing)).join('\n')).toContain('set "styles": "dist/main.css"');
		expect(readFileSync(join(missing, 'dist/main.css'), 'utf8')).toContain('red');

		const loaded = project({
			...files,
			'manifest.json': manifest({ main: 'dist/main.js', styles: 'dist/main.css' })
		});
		expect(await buildWidget(loaded)).toEqual([]);
	});

	it("warns when the widget's own code calls setTimeout or setInterval", async () => {
		const root = project({
			'manifest.json': manifest({ main: 'dist/main.js' }),
			'src/main.ts': [
				"import { poll } from './poll.js';",
				'const wait: number = 10;',
				'export default { mount() { window.setTimeout(poll, wait); } };'
			].join('\n'),
			'src/poll.js': 'export function poll() { setInterval(() => {}, 1000); }'
		});
		const warnings = (await buildWidget(root)).join('\n');
		expect(warnings).toContain('src/main.ts calls setTimeout');
		expect(warnings).toContain('use api.after instead');
		expect(warnings).toContain('src/poll.js calls setInterval');
		expect(warnings).toContain('use api.every instead');
	});

	it("doesn't warn for timers in dependencies, comments, strings or clears", async () => {
		const root = project({
			'manifest.json': manifest({ main: 'dist/main.js' }),
			'node_modules/dep/package.json': JSON.stringify({ name: 'dep', main: 'index.js' }),
			'node_modules/dep/index.js': 'export const later = (fn) => setTimeout(fn, 10);',
			'src/main.js': [
				"import { later } from 'dep';",
				'// setTimeout(fn) would not pause',
				'export default { mount(el) {',
				"	el.textContent = 'setInterval(';",
				'	clearTimeout(later(() => {}));',
				'} };'
			].join('\n')
		});
		expect(await buildWidget(root)).toEqual([]);
	});

	it('fails without an entry module', async () => {
		const root = project({ 'manifest.json': manifest({ main: 'dist/main.js' }) });
		await expect(buildWidget(root)).rejects.toThrow('No widget entry');
	});
});
