<script lang="ts">
	import Snowflake from '@lucide/svelte/icons/snowflake';
	import EdgeBackground from '#lib/components/edge/EdgeBackground.svelte';
	import PageCarousel from '#lib/components/edge/PageCarousel.svelte';
	import UpdatePrompt from '#lib/components/edge/UpdatePrompt.svelte';
	import { resolveShortcuts, type ShortcutAction } from '#lib/shortcuts/actions.ts';
	import {
		backgroundBlur,
		backgroundImage,
		display,
		freezeOnFullscreen,
		pageDirection,
		pages,
		pageTransition,
		shortcuts,
		shortcutsFailed,
		widgetBlur,
		widgetOpacity
	} from '#lib/state/settings.svelte.ts';
	import { widgetGlassStyle } from '#lib/theme/background.ts';
	import { bringEdgeToFront, placeEdgeWindow, setEdgeVisible } from '#lib/tauri/edge-window.ts';
	import { onReloadRequest, onShowPageRequest } from '#lib/tauri/events.ts';
	import { lendFocusToTextFields } from '#lib/tauri/focus.ts';
	import { watchFullscreenApp } from '#lib/tauri/fullscreen.ts';
	import { bindShortcuts } from '#lib/tauri/shortcuts.ts';
	import { setFrozen } from '#lib/widgets/api/timers.ts';
	import { watchWidgetChanges } from '#lib/widgets/revisions.svelte.ts';
	import { overlay } from '#lib/widgets/webview/motion.ts';

	// Its shortcut hides the Edge and shows it again. After a reload, the Edge is as it was.
	let hidden = $state(false);
	$effect(() => {
		placeEdgeWindow(display.current.monitor)
			.then(() => setEdgeVisible())
			.then((shows) => (hidden = !shows))
			.catch(console.error);
	});

	/** Hides the Edge, or shows it again above the windows that can cover it. */
	async function toggleEdge() {
		const shows = await setEdgeVisible(hidden);
		if (shows) await bringEdgeToFront();
		hidden = !shows;
	}

	$effect(() => onReloadRequest(() => location.reload()));

	$effect(() => watchWidgetChanges());

	// The keyboard stays with the app in front, also a game. Only a touched text field takes it.
	$effect(lendFocusToTextFields);

	// A file or link that the user drags in from another app opens in place of the Edge page if
	// no element takes it. A widget that takes a drop cancels `dragover` before this listener runs.
	$effect(() => {
		const refuse = (event: DragEvent) => {
			if (event.defaultPrevented) return;
			event.preventDefault();
			if (event.dataTransfer) event.dataTransfer.dropEffect = 'none';
		};
		window.addEventListener('dragover', refuse);
		return () => window.removeEventListener('dragover', refuse);
	});

	// Shortcuts work from any app, because the Edge is almost never the focused window. Settings
	// shows the shortcuts that another app has.
	let carousel: ReturnType<typeof PageCarousel>;
	const run: Record<ShortcutAction, () => void> = {
		previousPage: () => carousel.turn(-1),
		nextPage: () => carousel.turn(1),
		goToPage: () => carousel.find(),
		bringToFront: () => void bringEdgeToFront().catch(console.error),
		toggleEdge: () => void toggleEdge().catch(console.error)
	};
	$effect(() =>
		bindShortcuts(
			resolveShortcuts(shortcuts.current),
			(action) => run[action](),
			(failed) => {
				const before = shortcutsFailed.current;
				if (failed.length !== before.length || failed.some((a) => !before.includes(a))) {
					shortcutsFailed.current = failed;
				}
			}
		)
	);

	// A page tab picked in Settings > Pages turns the Edge to that page. It stays there after.
	$effect(() => onShowPageRequest((id) => carousel.show(id)));

	// When another app is full screen, widget timers and animations stop until it leaves full
	// screen. A touch on the Edge does not change this.
	let frozen = $state(false);
	$effect(() => {
		if (!freezeOnFullscreen.current) return;
		const stop = watchFullscreenApp((fullscreen) => (frozen = fullscreen));
		return () => {
			stop();
			frozen = false;
		};
	});
	// When the Edge is hidden, nobody sees the widgets. They stop to, but without the Paused badge.
	$effect(() => {
		const still = frozen || hidden;
		setFrozen(still);
		document.documentElement.classList.toggle('frozen', still);
	});

	// All widget cards read these values (WidgetSlot). They apply only over a background image.
	// On the plain color nothing shows through, so the cards stay opaque and there is no blur.
	const glass = $derived(
		backgroundImage.current
			? widgetGlassStyle(widgetOpacity.current, widgetBlur.current)
			: undefined
	);
</script>

<main class="relative h-screen w-screen overflow-hidden bg-background select-none" style={glass}>
	<!-- The background is behind the pages, because they are also positioned and come after it.
	     It shows betwen the widgets and on empty cells, and stays in place when the pages slide. -->
	{#if backgroundImage.current}
		<EdgeBackground image={backgroundImage.current} blur={backgroundBlur.current} />
	{/if}
	<PageCarousel
		bind:this={carousel}
		pages={pages.current}
		direction={pageDirection.current}
		transition={pageTransition.current}
	/>
	{#if frozen}
		<!-- Web views draw above the page, so they leave a hole for the badge (see `overlay`). -->
		<div
			{@attach overlay}
			class="fixed top-0 right-0 flex items-center gap-3 rounded-bl-xl bg-primary px-4 py-1.5 text-lg font-medium text-primary-foreground shadow-lg"
		>
			<Snowflake class="size-5" />Paused
		</div>
	{/if}
	<UpdatePrompt />
</main>
