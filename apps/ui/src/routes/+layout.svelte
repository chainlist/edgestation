<script lang="ts">
	import './layout.css';
	import { theme } from '#lib/state/settings.svelte.ts';
	import { applyTheme } from '#lib/theme/apply.ts';
	import { setWindowBackground } from '#lib/tauri/window-background.ts';
	import type { LayoutProps } from './$types';

	let { children }: LayoutProps = $props();

	// A seperate value, so a drag of the accent or radius slider does not call Tauri each time.
	const mode = $derived(theme.current.mode);

	$effect(() => applyTheme(document.documentElement, theme.current));
	$effect(() => void setWindowBackground(mode).catch(console.error));
</script>

{@render children()}
