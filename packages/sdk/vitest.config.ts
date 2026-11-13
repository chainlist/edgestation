import { svelte } from '@sveltejs/vite-plugin-svelte';
import { defineConfig } from 'vitest/config';

export default defineConfig({
	plugins: [svelte({ configFile: false, compilerOptions: { runes: true } })],
	// The server build of Svelte cannot mount components.
	resolve: { conditions: ['browser'] },
	test: {
		environment: 'happy-dom',
		include: ['test/**/*.spec.ts'],
		expect: { requireAssertions: true }
	}
});
