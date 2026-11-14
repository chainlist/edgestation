A widget built with the SDK goes from your source files to the `dist/main.js` EdgeStation loads through one Vite plugin. This page covers the build, loading the widget while you work on it, and shipping it.

## The Vite plugin

```js vite.config.js
import { widget } from '@edgestation/sdk/vite';
import { defineConfig } from 'vite';

export default defineConfig({ plugins: [widget()] });
```

Add your framework's Vite plugin next to it, as in `plugins: [react(), widget()]`. `widget()`:

- builds the entry into `dist/main.js`: one ES module with every dependency bundled in, your framework's runtime included,
- writes the CSS your code imports to `dist/main.css`,
- replaces `process.env.NODE_ENV`, which React and Vue read and a browser lacks, with `"production"` (or `"development"` when you build with `--mode development`),
- warns when `manifest.json` is missing, isn't valid JSON, or doesn't point at the files it writes: `main` should be `dist/main.js`, and `styles` should be `dist/main.css` when there is CSS,
- warns when a file of yours (not of your dependencies) calls `setTimeout` or `setInterval`, which don't pause with the widget: use [`api.after`](/docs/api/helpers#api-after) and [`api.every`](/docs/api/helpers#api-every) instead (see [Timers at a glance](/docs/lifecycle#timers-at-a-glance)).

It only applies to `vite build`. There is no dev server: EdgeStation loads the built files.

### Options

| Option  | Meaning                                                                                                                                               |
| ------- | ----------------------------------------------------------------------------------------------------------------------------------------------------- |
| `entry` | The entry module, relative to the project root. Defaults to the first of `src/main.ts`, `src/main.tsx`, `src/main.js` and `src/main.jsx` that exists. |

```js
widget({ entry: 'src/widget.ts' });
```

Vite's own options still apply: `build.outDir` changes `dist`, for example (then point the manifest at the new folder).

## Build

```bash
npx vite build
```

Scripts in `package.json` save typing:

```json package.json
{
	"type": "module",
	"scripts": {
		"build": "vite build",
		"dev": "vite build --watch"
	}
}
```

## Load it while you work

EdgeStation loads every folder of its widgets folder that has a `manifest.json`: `%APPDATA%\com.edgestation.dashboard\widgets\` on Windows, `~/.local/share/com.edgestation.dashboard/widgets/` on Linux (Settings → Widgets → Open folder). Rather than copying your project there, link it once. On Windows, from a Command Prompt:

```bat
mklink /J "%APPDATA%\com.edgestation.dashboard\widgets\my-widget" C:\path\to\my-widget
```

On Linux, from a terminal:

```bash
ln -s /path/to/my-widget ~/.local/share/com.edgestation.dashboard/widgets/my-widget
```

The link's name, `my-widget`, is the widget's id. Then:

1. In Settings → Widgets, click **Rescan**: the widget shows under **Your widgets**.
2. Add it to a page in Settings → Pages.
3. Run `npx vite build --watch`, which rebuilds on every change.
4. After each build, click **Reload Edge** in Settings → Widgets to load it.

**Rescan** reads the widget folders again, so Settings shows a new widget or a changed manifest. **Reload Edge** reloads the Edge, which then loads each widget's latest build.

## Share your widget

To share a widget, ship its folder with:

- `manifest.json`,
- `dist/` (or `main.js` and `styles.css` for a widget written by hand),
- the icon, and any file the widget reads with `api.asset()`.

Leave out `node_modules`, `src` and the build configuration: EdgeStation doesn't use them. A zip of those files is enough. To install it, a user extracts the folder into their widgets folder (Settings → Widgets → Open folder), clicks **Rescan**, and adds the widget to a page.

Set `version` in the manifest, and `minAppVersion` to the EdgeStation version the widget needs.

## Publish to the marketplace

Widgets in the marketplace show in Settings → Widgets → Browse, where a user installs one with a click. A new release never installs on its own: Settings → Widgets shows it under Installed, and the user installs it with Update or Update all. The list is `widgets.json` in the [edgestation-widgets](https://github.com/chainlist/edgestation-widgets) repository.

1. Keep the widget in a public GitHub repository, with its `manifest.json`, its icon and a `README.md` at the root: Browse shows the icon and the README from your default branch.
2. Publish each version as a GitHub release with two files attached:
   - `manifest.json`, the widget's manifest,
   - `widget.zip`, the files listed above, with `manifest.json` at the root of the zip rather than in a folder.
3. Raise `version` for every release. EdgeStation installs a release when its version is higher than the installed one, and skips pre-releases.
4. Open a pull request on edgestation-widgets that adds `{ "id": "my-widget", "repo": "you/my-widget" }` to `widgets.json`. The id becomes the widget's folder name and the key of every placement of it, so it can't change once listed.

EdgeStation reads the latest release's `manifest.json` to know whether there is a newer version, and downloads `widget.zip` only to install it. Browse shows how many times `widget.zip` was downloaded, all releases together, updates included.

This workflow builds the widget and publishes the release when you push a tag, such as `git tag 1.2.0 && git push --tags`:

```yaml .github/workflows/release.yml
name: Release
on:
  push:
    tags: ['*']
permissions:
  contents: write
jobs:
  release:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v5
      - uses: actions/setup-node@v5
        with:
          node-version: 22
      - run: npm ci
      - run: npm run build
      - run: zip -r widget.zip manifest.json dist icon.svg
      - run: gh release create "$GITHUB_REF_NAME" manifest.json widget.zip --title "$GITHUB_REF_NAME"
        env:
          GH_TOKEN: ${{ github.token }}
```

A widget is checked once, when it is added to the list, not at each release: every release you publish reaches its users the next time they open EdgeStation. A widget taken off the list stops getting updates.

Settings shows each marketplace widget with the permissions it asks for and the GitHub account that owns its repository. A verified badge marks trusted accounts. The badge never comes from `author` in the manifest, because any widget can set that.

## Working on the built-in widgets

The built-in widgets, in the repository's [`widgets` folder](https://github.com/chainlist/edgestation/tree/main/widgets), are Svelte 5 components built with the SDK the way a widget of your own is. Each is a pnpm workspace package, `@edgestation/widget-<id>`:

```text
clock/
  manifest.json   ("main": "dist/main.js", no "styles")
  package.json    (build and dev scripts)
  icon.svg
  src/
    main.ts
    Clock.svelte
  dist/main.js    (built, not committed)
```

They share one `widgets/vite.config.ts`. From the repository's root:

- `pnpm tauri dev` rebuilds the widgets whenever their source changes, and `pnpm tauri build` builds them before bundling the app.
- `pnpm --filter @edgestation/widget-clock build` builds one on its own.
- `pnpm check` type-checks them.
