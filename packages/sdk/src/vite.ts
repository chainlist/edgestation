// Node loads this file from the Vite config of the widget. Thus use no relative imports, and
// only erasable TypeScript.
import { existsSync, readFileSync } from 'node:fs';
import { join, relative, resolve } from 'node:path';
import type { Plugin } from 'vite';

export interface WidgetBuildOptions {
	/**
	 * The entry module, relative to the project root. The default is the first file that exists
	 * of `src/main.ts`, `src/main.tsx`, `src/main.js` and `src/main.jsx`.
	 */
	entry?: string;
}

const ENTRIES = ['src/main.ts', 'src/main.tsx', 'src/main.js', 'src/main.jsx'];

/** Writes a path as `manifest.json` does, with forward slashes and no leading `./`. */
const normalize = (path: string) => path.replace(/\\/g, '/').replace(/^\.\//, '');

/** What each global timer does that the `api` timer does not do. */
const TIMERS: Record<string, string> = {
	setInterval:
		"keeps running while the widget's page is off screen or the Edge is frozen, and after " +
		'the widget unmounts: use api.every instead',
	setTimeout:
		"fires even while the widget's page is off screen, and after the widget unmounts: use " +
		'api.after instead'
};
const GLOBALS = new Set(['window', 'globalThis', 'self']);

/** Matches the module id of a file of the widget, not a dependency, a virtual module or a query. */
const OWN = /^[^\0?]+\.(?:[cm]?[jt]sx?|svelte|vue)$/;

/** The parts of an ESTree node that `timersCalled` reads. */
interface Node {
	type?: string;
	name?: string;
	computed?: boolean;
	object?: Node;
	property?: Node;
	callee?: Node;
}

/** The global timer that `callee` names (`setTimeout`, `window.setInterval`, ...), if any. */
function timerName(callee: Node): string | undefined {
	const { object, property } = callee;
	const global =
		callee.type === 'MemberExpression' &&
		!callee.computed &&
		object?.type === 'Identifier' &&
		GLOBALS.has(object.name ?? '');
	const name = global ? property?.name : callee.type === 'Identifier' ? callee.name : undefined;
	return name !== undefined && Object.hasOwn(TIMERS, name) ? name : undefined;
}

/** The global timers that the code calls anywhere in `node`, an ESTree node or a list of nodes. */
function timersCalled(node: unknown, found = new Set<string>()): Set<string> {
	if (node && typeof node === 'object') {
		const { type, callee } = node as Node;
		const name = type === 'CallExpression' && callee ? timerName(callee) : undefined;
		if (name) found.add(name);
		for (const child of Object.values(node)) timersCalled(child, found);
	}
	return found;
}

/**
 * Builds the widget into what EdgeStation loads: one self-contained ES module, `dist/main.js`,
 * with all dependencies and frameworks in it. If the code imports CSS, it also writes
 * `dist/main.css`. Add the Vite plugin of your framework next to it:
 *
 * ```ts
 * export default defineConfig({ plugins: [react(), widget()] });
 * ```
 *
 * It also replaces `process.env.NODE_ENV`, because React and Vue read it and a browser does not
 * have it. It warns if `manifest.json` does not point to the files that it writes. It also warns
 * if the code of the widget, not of its dependencies, calls `setTimeout` or `setInterval`,
 * because these timers do not pause with the widget.
 */
export function widget(options: WidgetBuildOptions = {}): Plugin {
	let root = process.cwd();
	let outDir = 'dist';
	let manifest: { main?: unknown; styles?: unknown } | undefined;

	/** The path of a built file, as `manifest.json` must name it. */
	const built = (file: string) => normalize(relative(root, resolve(root, outDir, file)));

	return {
		name: 'edgestation-widget',
		apply: 'build',

		config(config, { mode }) {
			const base = resolve(config.root ?? process.cwd());
			const entry = options.entry ?? ENTRIES.find((file) => existsSync(join(base, file)));
			if (!entry) {
				throw new Error(
					`No widget entry: create one of ${ENTRIES.join(', ')}, or pass widget({ entry })`
				);
			}
			const nodeEnv = mode === 'production' ? 'production' : 'development';
			return {
				define:
					config.define?.['process.env.NODE_ENV'] === undefined
						? { 'process.env.NODE_ENV': JSON.stringify(nodeEnv) }
						: {},
				build: {
					lib: { entry, formats: ['es'], fileName: () => 'main.js', cssFileName: 'main' },
					outDir: config.build?.outDir ?? 'dist'
				}
			};
		},

		configResolved(config) {
			root = config.root;
			outDir = config.build.outDir;
		},

		buildStart() {
			const file = join(root, 'manifest.json');
			manifest = undefined;
			if (!existsSync(file)) {
				this.warn('No manifest.json in the project root: EdgeStation only lists folders with one.');
				return;
			}
			try {
				manifest = JSON.parse(readFileSync(file, 'utf8'));
			} catch (error) {
				this.warn(`manifest.json is not valid JSON: ${(error as Error).message}`);
				return;
			}
			const main = typeof manifest?.main === 'string' ? manifest.main : 'main.js';
			if (normalize(main) !== built('main.js')) {
				this.warn(
					`manifest.json has "main": "${main}" but the build writes ${built('main.js')}: ` +
						`set "main": "${built('main.js')}".`
				);
			}
		},

		// Run last, so TypeScript, Svelte and Vue files are plain JavaScript at this time, in any
		// plugin order. Do not check dependencies, because their timers are their own concern.
		transform: {
			order: 'post',
			handler(code, id) {
				if (!OWN.test(id) || id.includes('/node_modules/')) return;
				if (!/\bset(?:Timeout|Interval)\b/.test(code)) return;
				for (const name of timersCalled(this.parse(code))) {
					this.warn(`${normalize(relative(root, id))} calls ${name}, which ${TIMERS[name]}.`);
				}
			}
		},

		// Not `generateBundle`, because the Vite CSS plugin adds main.css to the bundle after it.
		writeBundle(_, bundle) {
			const css = built('main.css');
			const styles = typeof manifest?.styles === 'string' ? normalize(manifest.styles) : undefined;
			if (manifest && bundle['main.css'] && styles !== css) {
				this.warn(
					`The build writes ${css}: set "styles": "${css}" in manifest.json so EdgeStation ` +
						`loads it into the widget.`
				);
			}
		}
	};
}
