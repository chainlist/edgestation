This page writes one widget twice: a tile that shows the CPU load, first by hand in plain JavaScript, then with the SDK and a build step. You need EdgeStation installed and running.

## By hand

### 1. Create the folder

Open the widgets folder: open Settings from the EdgeStation icon in the notification area (click it on Windows, choose **Settings** in its menu on Linux), then **Widgets → Open folder**. Starting EdgeStation again also opens Settings. The folder is `%APPDATA%\com.edgestation.dashboard\widgets\` on Windows and `~/.local/share/com.edgestation.dashboard/widgets/` on Linux. Create a folder named `cpu` in it: that name is the widget's id.

### 2. Write the manifest

```json cpu/manifest.json
{
	"name": "CPU",
	"description": "The load of all CPU cores.",
	"minAppVersion": "0.1.0",
	"styles": "styles.css",
	"size": { "w": 1, "h": 1 },
	"permissions": ["system"]
}
```

Only `name` is required. `size` is how many grid cells the widget takes when it is added to a page, and `minAppVersion` is the oldest EdgeStation it runs on. `permissions` lists the parts of the `api` the widget uses: `system` lets it call `api.system()`. The [manifest page](/docs/manifest) lists every field.

### 3. Write the module

```js cpu/main.js
export default {
	mount(el, api) {
		api.every(1000, async () => {
			const { cpu } = await api.system();
			el.textContent = `${Math.round(cpu.load)} %`;
		});
	}
};
```

`mount` receives `el`, the element to draw into, and the [`api`](/docs/api). [`api.every`](/docs/api/helpers#api-every) is a `setInterval` that EdgeStation manages: it stops while the widget's page is off screen, and is cleared when the widget unmounts. [`api.system()`](/docs/api/system) reads the machine when called.

### 4. Style it

```css cpu/styles.css
.widget {
	display: grid;
	place-items: center;
	color: var(--primary);
	font-size: 30cqh;
	font-weight: 600;
}
```

`el` has the class `widget` and fills its grid cell. It is a size container, so `cqh` (a percent of the cell's height) scales the text with the cell, and `--primary` is the accent color the user picked. See [Styling and theming](/docs/styling).

### 5. Put it on a page

1. In Settings → Widgets, click **Rescan**. CPU shows under **Your widgets**.
2. In Settings → Pages, open a page, choose CPU in its widget list and click **Add**. Drag it to move it, drag a corner to resize it.

The tile shows on the Edge right away. After you edit `main.js` or `styles.css`, click **Reload Edge** in Settings → Widgets to load the change.

> If something is wrong, for example a typo that throws in `mount`, an error card replaces the tile with the message and a Retry button. The rest of the Edge keeps running.

## With the SDK

The SDK is for widgets written in TypeScript or a framework, or that use packages from npm. It needs [Node.js](https://nodejs.org) and a package manager; the commands below use npm.

### 1. Create the project

```bash
mkdir cpu
cd cpu
npm init -y
npm pkg set type=module
npm install -D @edgestation/sdk vite
```

### 2. Add the manifest and the build

```json manifest.json
{
	"$schema": "./node_modules/@edgestation/sdk/manifest.schema.json",
	"name": "CPU",
	"description": "The load of all CPU cores.",
	"minAppVersion": "0.1.0",
	"main": "dist/main.js",
	"styles": "dist/main.css",
	"size": { "w": 1, "h": 1 },
	"permissions": ["system"]
}
```

```js vite.config.js
import { widget } from '@edgestation/sdk/vite';
import { defineConfig } from 'vite';

export default defineConfig({ plugins: [widget()] });
```

`$schema` points your editor at the SDK's schema, which completes and checks the manifest as you type. `main` and `styles` point at what the build writes.

### 3. Write the widget

```js src/main.js
import { defineWidget } from '@edgestation/sdk';
import './styles.css';

export default defineWidget({
	mount(el, api) {
		api.every(1000, async () => {
			const { cpu } = await api.system();
			el.textContent = `${Math.round(cpu.load)} %`;
		});
	}
});
```

Put the stylesheet from step 4 above in `src/styles.css`. `defineWidget` returns its argument unchanged: it is there so your editor knows the types of `el` and `api`, in plain JavaScript too. Name the file `src/main.ts` to write TypeScript instead.

### 4. Build and link it

```bash
npx vite build
```

The build writes `dist/main.js` and `dist/main.css`. Rather than copying the project into the widgets folder after each build, link it there once. On Windows, from a Command Prompt:

```bat
mklink /J "%APPDATA%\com.edgestation.dashboard\widgets\cpu" C:\path\to\cpu
```

On Linux, from a terminal:

```bash
ln -s /path/to/cpu ~/.local/share/com.edgestation.dashboard/widgets/cpu
```

Then click **Rescan** in Settings → Widgets and add the widget to a page, as above.

### 5. Work on it

Rebuild on every change:

```bash
npx vite build --watch
```

and click **Reload Edge** in Settings → Widgets to load each new build.

## Next steps

- Use a framework: [Svelte, React and Vue](/docs/frameworks).
- Let the user choose what the widget shows: [Settings](/docs/settings).
- See what else the widget can read and do: [The api object](/docs/api).
- Ship it to others: [Build, test and share](/docs/build#share-your-widget).
