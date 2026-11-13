// Writes THIRD-PARTY-NOTICES.txt: the license texts and copyright notices of the third-party code
// in the installers. The installers ship it next to LICENSE.txt (bundle.resources in
// apps/desktop/tauri.conf.json).
//
//   pnpm notices           writes the file again
//   pnpm notices --check   fails if the file is out of date (notices-check.yml runs it)
//
// It lists three kinds of components:
// - The Rust crates linked into the program, on each target of the release build
//   (desktop-release.yml). `cargo metadata` gives the dependency graph. The normal dependencies
//   count, proc macros included, since their output is compiled in. Build and dev dependencies
//   do not, as their code only runs on the build machine.
// - Other code that one of these crates links into the program (EXTRA below).
// - The npm packages bundled into the UI (apps/ui) and the built-in widgets (widgets/*). The
//   script builds each of them with Vite, and a plugin notes the files from node_modules that
//   end up in the output: modules with code in a chunk, CSS that Tailwind inlines, and assets
//   such as fonts. The builds write their usual output folders.
//
// EdgeStation's own packages (the desktop crate and @edgestation/*) are left out: LICENSE and
// widgets/LICENSE cover them. A package without a license file of its own takes the license
// files of a crate from the same repository and with the same license, published from the
// repository root. Otherwise it gets the standard text of its license from licenses/.

import { execFileSync } from 'node:child_process';
import { existsSync, readdirSync, readFileSync, writeFileSync } from 'node:fs';
import { basename, dirname, join, relative, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { createBuilder } from 'vite';

const ROOT = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const OUTPUT = join(ROOT, 'THIRD-PARTY-NOTICES.txt');
const STANDARD_TEXTS = join(ROOT, 'scripts/notices/licenses');
const EXTRA_TEXTS = join(ROOT, 'scripts/notices/extra');

// The targets of the release build, with the name to show for each. A new platform in
// desktop-release.yml needs its target here too.
const TARGETS = [
	{ triple: 'x86_64-pc-windows-msvc', name: 'Windows' },
	{ triple: 'x86_64-unknown-linux-gnu', name: 'Linux' }
];

// Third-party code that a crate links into the program, but that `cargo metadata` does not list.
// Each one has its license text in extra/. `version` reads the version that the crate brings.
// When it differs from `checked`, the version whose license was read for the text, the script
// stops: read the license of the new version, update the text and `checked`.
const EXTRA = [
	{
		// On MSVC, webview2-com-sys links Microsoft's WebView2LoaderStatic.lib from its x64 folder.
		// The text is the license file of the Microsoft.Web.WebView2 NuGet package, from
		// https://www.nuget.org/packages/Microsoft.Web.WebView2/1.0.3800.47/License
		crate: 'webview2-com-sys',
		name: 'Microsoft WebView2 SDK loader',
		license: 'BSD-style, see the notice',
		source: (version) => `https://www.nuget.org/packages/Microsoft.Web.WebView2/${version}`,
		version: (dir) => fileVersion(join(dir, 'x64/WebView2Loader.dll')),
		checked: '1.0.3800.47',
		text: 'webview2-sdk.txt'
	}
];

// The files whose text the notices reproduce, at the root of a package, or in a folder of
// licenses (as REUSE lays them out).
const LICENSE_FILE = /^(un)?licen[cs]e|^copying|^copyright|^notice/i;
const LICENSE_DIR = /^licen[cs]es?$/i;

const slash = (path) => path.split('\\').join('/');
const inNodeModules = (path) => slash(path).includes('/node_modules/');
const byName = (a, b) => a.localeCompare(b, 'en');

/** The text of a license file, with LF line ends and no trailing spaces or blank lines. */
function normalize(text) {
	return text
		.replace(/^\uFEFF/, '')
		.replace(/\r\n?/g, '\n')
		.split('\n')
		.map((line) => line.trimEnd())
		.join('\n')
		.replace(/^\n+|\n+$/g, '');
}

/** The license files at the root of `dir`, and in a licenses folder there, sorted by name. */
function licenseFiles(dir) {
	const files = [];
	for (const entry of readdirSync(dir, { withFileTypes: true })) {
		const path = join(dir, entry.name);
		if (entry.isFile() && LICENSE_FILE.test(entry.name)) files.push(path);
		if (entry.isDirectory() && LICENSE_DIR.test(entry.name)) {
			for (const inner of readdirSync(path, { withFileTypes: true })) {
				if (inner.isFile()) files.push(join(path, inner.name));
			}
		}
	}
	return files.sort((a, b) => byName(slash(a), slash(b)));
}

/** A browsable URL for the `repository` field of a package.json or Cargo.toml. */
function repositoryUrl(repository) {
	let url = typeof repository === 'string' ? repository : repository?.url;
	if (!url) return undefined;
	if (/^[\w.-]+\/[\w.-]+$/.test(url)) url = `https://github.com/${url}`;
	return url
		.replace(/^github:/, 'https://github.com/')
		.replace(/^git\+/, '')
		.replace(/^git:\/\//, 'https://')
		.replace(/^ssh:\/\/git@/, 'https://')
		.replace(/^git@([^:]+):/, 'https://$1/')
		.replace(/\.git$/, '');
}

// Rust crates

/** The `.cargo_vcs_info.json` of a published crate: its commit and its folder in the repo. */
function vcsInfo(dir) {
	const file = join(dir, '.cargo_vcs_info.json');
	if (!existsSync(file)) return {};
	const info = JSON.parse(readFileSync(file, 'utf8'));
	return { commit: info.git?.sha1, path: info.path_in_vcs ?? '' };
}

/** The third-party crates linked into the program on each target, by package id. */
function rustCrates() {
	const crates = new Map();
	for (const target of TARGETS) {
		const metadata = JSON.parse(
			execFileSync(
				'cargo',
				[
					'metadata',
					'--format-version=1',
					'--locked',
					`--filter-platform=${target.triple}`,
					'--manifest-path',
					join(ROOT, 'apps/desktop/Cargo.toml')
				],
				{ encoding: 'utf8', maxBuffer: 1 << 28, stdio: ['ignore', 'pipe', 'inherit'] }
			)
		);
		const packages = new Map(metadata.packages.map((pkg) => [pkg.id, pkg]));
		const nodes = new Map(metadata.resolve.nodes.map((node) => [node.id, node]));
		const stack = [metadata.resolve.root];
		const seen = new Set();
		while (stack.length) {
			const id = stack.pop();
			if (seen.has(id)) continue;
			seen.add(id);
			const pkg = packages.get(id);
			// Workspace and path crates have no source. That is EdgeStation itself.
			if (pkg.source !== null) {
				const crate = crates.get(id) ?? { pkg, platforms: new Set() };
				crate.platforms.add(target.name);
				crates.set(id, crate);
			}
			for (const dep of nodes.get(id).deps) {
				if (dep.dep_kinds.some((kind) => kind.kind === null)) stack.push(dep.pkg);
			}
		}
	}

	const components = [...crates.values()].map(({ pkg, platforms }) => {
		const dir = dirname(pkg.manifest_path);
		const files = licenseFiles(dir);
		if (pkg.license_file) files.push(resolve(dir, pkg.license_file));
		return {
			kind: 'rust',
			name: pkg.name,
			version: pkg.version,
			license: pkg.license ?? 'not declared, see the notices',
			source: pkg.repository ?? `https://crates.io/crates/${pkg.name}`,
			authors: pkg.authors ?? [],
			repository: pkg.repository,
			dir,
			vcs: vcsInfo(dir),
			platforms:
				platforms.size === TARGETS.length
					? undefined
					: TARGETS.filter((t) => platforms.has(t.name)).map((t) => t.name),
			files: [...new Set(files)]
		};
	});

	// A crate published from a folder of its repository may leave out the license files at the
	// repository root. Take them from the crate at that root, when the tree has it.
	for (const crate of components) {
		if (crate.files.length) continue;
		const sibling = components.find(
			(other) =>
				other !== crate &&
				other.files.length &&
				other.vcs.path === '' &&
				other.license === crate.license &&
				((other.vcs.commit && other.vcs.commit === crate.vcs.commit) ||
					(other.repository && other.repository === crate.repository))
		);
		if (sibling) {
			crate.files = sibling.files;
			crate.filesFrom = `${sibling.name} ${sibling.version}`;
		}
	}
	return components;
}

/** The file version of a Windows DLL, from the VS_FIXEDFILEINFO of its version resource. */
function fileVersion(file) {
	const bytes = readFileSync(file);
	const at = bytes.indexOf(Buffer.from([0xbd, 0x04, 0xef, 0xfe]));
	if (at < 0) throw new Error(`${file} has no version resource`);
	const high = bytes.readUInt32LE(at + 8);
	const low = bytes.readUInt32LE(at + 12);
	return [high >>> 16, high & 0xffff, low >>> 16, low & 0xffff].join('.');
}

/** The components of EXTRA whose crate is in `crates`, on the same platforms as the crate. */
function extraComponents(crates) {
	const components = [];
	for (const extra of EXTRA) {
		for (const crate of crates.filter((c) => c.name === extra.crate)) {
			const version = extra.version(crate.dir);
			if (version !== extra.checked) {
				throw new Error(
					`${crate.name} ${crate.version} brings ${extra.name} ${version}, but the license ` +
						`text in scripts/notices/extra/${extra.text} is the one of ${extra.checked}. ` +
						`Read the license of ${version} at ${extra.source(version)}, update the text if ` +
						`it changed, and set \`checked\` to ${version} in scripts/notices/generate.mjs.`
				);
			}
			components.push({
				kind: 'extra',
				name: extra.name,
				version,
				license: extra.license,
				source: extra.source(version),
				authors: [],
				platforms: crate.platforms,
				note: `linked into the program by ${crate.name} ${crate.version}`,
				files: [join(EXTRA_TEXTS, extra.text)]
			});
		}
	}
	return components;
}

// npm packages

/** The nearest package.json above `file` that names a package, with its folder. */
function packageOf(file, cache) {
	let dir = dirname(file);
	while (basename(dir) !== 'node_modules' && dirname(dir) !== dir) {
		if (!cache.has(dir)) {
			const path = join(dir, 'package.json');
			const data = existsSync(path) ? JSON.parse(readFileSync(path, 'utf8')) : undefined;
			cache.set(dir, data?.name && data.version ? data : undefined);
		}
		const data = cache.get(dir);
		if (data) return { dir, data };
		dir = dirname(dir);
	}
	return undefined;
}

/**
 * A Vite plugin that adds to `files` the files from node_modules that a client build puts in its
 * output. Tailwind reads the CSS files of `@import` itself, so they are not modules: the plugin
 * notes the files that a transform reports with `addWatchFile`, which Tailwind does for each one.
 */
function bundledFiles(files) {
	const note = (file) => {
		if (inNodeModules(file)) files.add(resolve(file));
	};
	return {
		name: 'edgestation-notices',
		configResolved(config) {
			for (const plugin of config.plugins) {
				const hook = plugin.transform;
				const handler = typeof hook === 'function' ? hook : hook?.handler;
				if (!handler || Object.isFrozen(plugin) || (hook !== handler && Object.isFrozen(hook))) {
					continue;
				}
				const wrapped = function (...args) {
					const context = this;
					const proxy = new Proxy(context, {
						get(target, key) {
							if (key === 'addWatchFile') {
								return (file) => {
									if (target.environment?.name === 'client') note(file);
									return target.addWatchFile(file);
								};
							}
							const value = Reflect.get(target, key);
							return typeof value === 'function' ? value.bind(target) : value;
						}
					});
					return handler.apply(proxy, args);
				};
				if (hook === handler) plugin.transform = wrapped;
				else hook.handler = wrapped;
			}
		},
		generateBundle(options, bundle) {
			if (this.environment.name !== 'client') return;
			const root = this.environment.config.root;
			for (const output of Object.values(bundle)) {
				if (output.type === 'asset') {
					for (const name of output.originalFileNames ?? []) note(resolve(root, name));
					continue;
				}
				for (const id of output.moduleIds) {
					if (id.startsWith('\0')) continue;
					const file = id.split('?')[0];
					// A module that keeps no code adds nothing, unless it is CSS, whose code goes to
					// a CSS file.
					const rendered = output.modules[id]?.renderedLength ?? 1;
					if (rendered > 0 || /\.(css|less|sass|scss|styl|stylus|pcss|postcss)$/.test(file)) {
						note(file);
					}
				}
			}
		}
	};
}

/** Builds the app with Vite from `dir`, as `vite build` does, and notes the bundled files. */
async function build(dir, configFile, files) {
	const cwd = process.cwd();
	process.chdir(dir);
	try {
		// `null` picks the builder as `vite build` does. The default builds every environment, and
		// the SSR build of a widget then writes over its dist/main.js with the server code of Svelte.
		const builder = await createBuilder(
			{
				root: dir,
				configFile,
				logLevel: 'error',
				plugins: [bundledFiles(files)]
			},
			null
		);
		await builder.buildApp();
	} finally {
		process.chdir(cwd);
	}
}

/** The npm packages in the bundles of the UI and the built-in widgets. */
async function npmPackages() {
	const builds = [{ name: 'interface', dir: join(ROOT, 'apps/ui') }];
	for (const entry of readdirSync(join(ROOT, 'widgets'), { withFileTypes: true }).sort((a, b) =>
		byName(a.name, b.name)
	)) {
		const dir = join(ROOT, 'widgets', entry.name);
		if (entry.isDirectory() && existsSync(join(dir, 'package.json'))) {
			builds.push({
				name: `${entry.name} widget`,
				dir,
				configFile: join(ROOT, 'widgets/vite.config.ts')
			});
		}
	}

	const cache = new Map();
	const packages = new Map();
	for (const { name, dir, configFile } of builds) {
		console.log(`Building ${relative(ROOT, dir)}`);
		const files = new Set();
		await build(dir, configFile ?? join(dir, 'vite.config.ts'), files);
		for (const file of files) {
			const found = packageOf(file, cache);
			if (!found || found.data.name.startsWith('@edgestation/')) continue;
			const key = `${found.data.name}@${found.data.version}`;
			const pkg = packages.get(key) ?? { ...found, usedBy: new Set() };
			pkg.usedBy.add(name);
			packages.set(key, pkg);
		}
	}

	return [...packages.values()].map(({ dir, data, usedBy }) => {
		const license =
			typeof data.license === 'string'
				? data.license
				: (data.licenses ?? [])
						.map((entry) => (typeof entry === 'string' ? entry : entry.type))
						.join(' OR ') || 'not declared, see the notices';
		const author = typeof data.author === 'string' ? data.author : data.author?.name;
		return {
			kind: 'npm',
			name: data.name,
			version: data.version,
			license,
			source:
				repositoryUrl(data.repository) ??
				data.homepage ??
				`https://www.npmjs.com/package/${data.name}`,
			authors: author ? [author] : [],
			usedBy: [...usedBy],
			files: licenseFiles(dir)
		};
	});
}

// Output

/**
 * The standard text of a license that a package declares without a license file. For a choice
 * of licenses (`A OR B`), the first one that licenses/ has.
 */
function standardText(component) {
	const choices = component.license
		.split(/\s+OR\s+|\//)
		.map((id) => id.replace(/[()]/g, '').trim());
	for (const id of choices) {
		const file = join(STANDARD_TEXTS, `${id}.txt`);
		if (!existsSync(file)) continue;
		// The template copyright line of the MIT text has nothing to fill it with here.
		const text = normalize(readFileSync(file, 'utf8'))
			.replace(/^Copyright \(c\) <year> <copyright holders>\n+/m, '')
			.trim();
		return { id, text };
	}
	throw new Error(
		`${component.name} ${component.version} (${component.license}) has no license file, and ` +
			`scripts/notices/licenses/ has no standard text for its license. Add one there.`
	);
}

const label = (c) => `${c.name} ${c.version}`;
const RULE = '-'.repeat(79);

function title(text) {
	return `${text}\n${'='.repeat(text.length)}\n`;
}

/** Wraps a list of names into lines of at most 79 characters, after `prefix`. */
function wrapList(prefix, items) {
	const lines = [];
	let line = prefix;
	for (const [index, item] of items.entries()) {
		const piece = item + (index < items.length - 1 ? ',' : '');
		if (line.length + 1 + piece.length > 79 && line.trim().length > prefix.trim().length) {
			lines.push(line);
			line = ' '.repeat(prefix.length) + piece;
		} else {
			line += (line === prefix ? '' : ' ') + piece;
		}
	}
	lines.push(line);
	return lines.join('\n');
}

function render(rust, extra, npm) {
	const all = [...rust, ...extra, ...npm];
	// Group the texts, so that each one appears once, with all the components it applies to.
	const groups = new Map();
	for (const component of all) {
		component.texts = [];
		const texts = component.files
			.map((file) => normalize(readFileSync(file, 'utf8')))
			.filter(Boolean);
		if (!texts.length) {
			const standard = standardText(component);
			component.standard = standard.id;
			texts.push(standard.text);
		}
		for (const text of texts) {
			const group = groups.get(text) ?? { text, components: [] };
			group.components.push(component);
			groups.set(text, group);
			component.texts.push(group);
		}
	}
	const ordered = [...groups.values()].map((group) => ({
		...group,
		components: group.components.sort((a, b) => byName(label(a), label(b)))
	}));
	ordered.sort(
		(a, b) => byName(label(a.components[0]), label(b.components[0])) || byName(a.text, b.text)
	);
	ordered.forEach((group, index) => (group.number = index + 1));
	const numberOf = new Map(ordered.map((group) => [group.text, group.number]));

	const entry = (component) => {
		const lines = [
			label(component),
			`  License: ${component.license}`,
			`  Source: ${component.source}`
		];
		if (component.platforms) lines.push(`  Only on: ${component.platforms.join(', ')}`);
		if (component.usedBy) lines.push(wrapList('  In: ', component.usedBy));
		if (component.note) lines.push(`  Note: ${component.note}`);
		const numbers = [...new Set(component.texts.map((group) => numberOf.get(group.text)))].sort(
			(a, b) => a - b
		);
		let notices = `  Notices: ${numbers.map((n) => `[${n}]`).join(' ')}`;
		if (component.filesFrom) notices += ` (the license files of ${component.filesFrom})`;
		if (component.standard) {
			notices += ` (standard ${component.standard} text: the package has no license file)`;
		}
		lines.push(notices);
		if (component.standard && component.authors.length) {
			lines.push(wrapList('  Authors: ', component.authors));
		}
		return lines.join('\n');
	};

	const sorted = (list) => [...list].sort((a, b) => byName(label(a), label(b)));
	const parts = [
		title('EdgeStation third-party notices'),
		'EdgeStation includes the third-party software listed below. Each component is\n' +
			'under its own license. The license texts and copyright notices that come with\n' +
			'the components follow the lists, numbered, and each component names the\n' +
			'numbers that apply to it. These licenses cover those components only.\n\n' +
			'This file is generated by `pnpm notices` (scripts/notices/generate.mjs) in the\n' +
			'EdgeStation repository. Do not edit it by hand.\n',
		title(`Rust crates in the EdgeStation program (${rust.length})`),
		sorted(rust).map(entry).join('\n\n') + '\n',
		title(`Other code in the EdgeStation program (${extra.length})`),
		sorted(extra).map(entry).join('\n\n') + '\n',
		title(`npm packages in the interface and the built-in widgets (${npm.length})`),
		sorted(npm).map(entry).join('\n\n') + '\n',
		title(`License texts and copyright notices (${ordered.length})`),
		ordered
			.map((group) => {
				const head = [RULE, `[${group.number}]`, wrapList('For: ', group.components.map(label))];
				const standard = group.components.filter((component) => component.standard);
				if (standard.length === group.components.length) {
					head.push(
						'These packages declare this license but include no license file. This is\n' +
							'the standard text of the license. The copyright is that of their authors.'
					);
				} else if (standard.length) {
					head.push(
						wrapList('Standard text for: ', standard.map(label)) +
							'\n(no license file in these packages: the copyright is that of their authors)'
					);
				}
				return `${head.join('\n')}\n\n${group.text}\n`;
			})
			.join('\n')
	];
	return parts.join('\n');
}

/** The names of the components that a notices file lists. */
function componentsIn(text) {
	const lines = text.split('\n');
	return new Set(lines.filter((line, i) => lines[i + 1]?.startsWith('  License: ')));
}

const check = process.argv.includes('--check');
const npm = await npmPackages();
console.log('Reading the Rust dependency graph');
const rust = rustCrates();
const extra = extraComponents(rust);
const text = normalize(render(rust, extra, npm)) + '\n';

if (check) {
	const current = existsSync(OUTPUT) ? normalize(readFileSync(OUTPUT, 'utf8')) + '\n' : '';
	if (current !== text) {
		const before = componentsIn(current);
		const after = componentsIn(text);
		const added = [...after].filter((name) => !before.has(name));
		const removed = [...before].filter((name) => !after.has(name));
		console.error('THIRD-PARTY-NOTICES.txt is out of date.');
		if (added.length) console.error(`New components: ${added.join(', ')}`);
		if (removed.length) console.error(`Components that left: ${removed.join(', ')}`);
		if (!added.length && !removed.length) console.error('A license text or detail changed.');
		console.error('Run `pnpm notices` and commit THIRD-PARTY-NOTICES.txt.');
		process.exit(1);
	}
	console.log(
		`THIRD-PARTY-NOTICES.txt is up to date (${rust.length} crates, ${npm.length} npm packages).`
	);
} else {
	writeFileSync(OUTPUT, text);
	console.log(`Wrote THIRD-PARTY-NOTICES.txt (${rust.length} crates, ${npm.length} npm packages).`);
}
