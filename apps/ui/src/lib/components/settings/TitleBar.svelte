<!--
	The title bar of the settings window, which has no frame of its own (see `open_settings` in
	Rust). It looks the same on Windows and Linux. A drag on it moves the window, and a double
	click maximizes or restores it (Tauri's drag region). Its buttons stay out of the Tab order,
	as the buttons of a system title bar do.
-->
<script lang="ts">
	import { getCurrentWindow } from '@tauri-apps/api/window';

	const win = getCurrentWindow();

	let maximized = $state(false);
	/** The window is active. Like a system title bar, the buttons fade when it is not. */
	let focused = $state(true);

	// Maximize, restore, Win+Up and a drag to the screen top all resize the window, so the
	// maximize button reads the state again after each resize.
	$effect(() => {
		const read = () => win.isMaximized().then((on) => (maximized = on), console.error);
		read();
		const offs = [win.onResized(read), win.onFocusChanged(({ payload }) => (focused = payload))];
		return () => offs.forEach((off) => void off.then((stop) => stop()));
	});

	const run = (action: () => Promise<void>) => () => void action().catch(console.error);

	const shape = 'grid h-full w-12 place-items-center transition-colors duration-100 outline-none';
	const button = `${shape} hover:bg-foreground/10 hover:text-foreground active:bg-foreground/15`;
</script>

<!-- The whole bar is a drag region, except its buttons. -->
<header
	data-tauri-drag-region="deep"
	class={[
		'flex h-10 shrink-0 justify-end select-none',
		focused ? 'text-foreground/80' : 'text-muted-foreground/60'
	]}
>
	<button
		class={button}
		tabindex="-1"
		aria-label="Minimize"
		title="Minimize"
		onclick={run(() => win.minimize())}
	>
		<svg viewBox="0 0 10 10" class="size-2.5" aria-hidden="true">
			<path d="M0 5.5h10" stroke="currentColor" />
		</svg>
	</button>
	<button
		class={button}
		tabindex="-1"
		aria-label={maximized ? 'Restore' : 'Maximize'}
		title={maximized ? 'Restore' : 'Maximize'}
		onclick={run(() => win.toggleMaximize())}
	>
		<svg viewBox="0 0 10 10" class="size-2.5" fill="none" stroke="currentColor" aria-hidden="true">
			{#if maximized}
				<!-- Two windows, one behind the other. -->
				<rect x="0.5" y="2.5" width="7" height="7" rx="1" />
				<path d="M2.5 2.5v-1a1 1 0 0 1 1-1h5a1 1 0 0 1 1 1v5a1 1 0 0 1-1 1h-1" />
			{:else}
				<rect x="0.5" y="0.5" width="9" height="9" rx="1" />
			{/if}
		</svg>
	</button>
	<!-- Red on hover, as Windows and most Linux desktops show Close. -->
	<button
		class="{shape} hover:bg-[#c42b1c] hover:text-white active:bg-[#c42b1c]/85"
		tabindex="-1"
		aria-label="Close"
		title="Close"
		onclick={run(() => win.close())}
	>
		<svg viewBox="0 0 10 10" class="size-2.5" aria-hidden="true">
			<path d="M0.5 0.5l9 9M9.5 0.5l-9 9" stroke="currentColor" />
		</svg>
	</button>
</header>
