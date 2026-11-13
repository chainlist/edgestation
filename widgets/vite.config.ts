import { widget } from '@edgestation/sdk/vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';
import { defineConfig } from 'vite';

/**
 * Builds the widget in the current folder. The package.json of each built-in widget runs this.
 * The build is the same as for each widget made with `@edgestation/sdk`: `src/main.ts` becomes
 * one `dist/main.js` file that contains the Svelte runtime. Components add their styles to the
 * shadow root of the widget when they mount (`emitCss: false`), so there is no CSS file.
 */
export default defineConfig({
	plugins: [
		svelte({ configFile: false, emitCss: false, compilerOptions: { runes: true } }),
		widget()
	],
	// `pnpm tauri dev` runs the watch builds in the same terminal as Tauri. Do not clear the Tauri
	// output.
	clearScreen: false
});
