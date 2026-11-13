<script lang="ts">
	import { onNavigate } from '$app/navigation';
	import { prefersReducedMotion } from 'svelte/motion';
	import SettingsNav, { SECTIONS } from '#lib/components/settings/SettingsNav.svelte';
	import TitleBar from '#lib/components/settings/TitleBar.svelte';
	import { settingsTransition } from '#lib/components/settings/transition.ts';
	import { overlayScrollbars } from '#lib/scrollbars.ts';
	import type { LayoutProps } from './$types';

	let { children }: LayoutProps = $props();

	let main = $state<HTMLElement>();

	onNavigate(({ from, to, complete }) => {
		if (!from || !to || !main) return;
		const type = settingsTransition(from.url.pathname, to.url.pathname, SECTIONS);
		if (!type) return;

		// `main` scrolls, not the window, so the scroll handler of SvelteKit does not get to it. New
		// content starts at the top. Back on the page list, the page that the user left scrolls into
		// view.
		const content = main;
		const scroll = () => {
			content.scrollTop = 0;
			const id = type === 'back' && from.params?.id;
			if (id) content.querySelector(`[data-page="${id}"]`)?.scrollIntoView({ block: 'nearest' });
		};
		// Older WebKitGTK versions (Linux) have no view transitions.
		if (prefersReducedMotion.current || !('startViewTransition' in document)) return scroll;

		return new Promise((resolve) => {
			document.startViewTransition({
				update: async () => {
					resolve();
					await complete;
					scroll();
				},
				types: [type]
			});
		});
	});
</script>

<!-- The window has no frame. The sidebar runs to the top, where its header is the left part of
     the title bar, and the content column has the rest of it, with the window buttons. -->
<div class="flex h-screen">
	<SettingsNav />
	<div class="flex min-w-0 flex-1 flex-col">
		<TitleBar />
		<!-- Holds the overlay scrollbars of `main`, which fills it. OverlayScrollbars resets the
		     padding of the viewport, so the padding goes on an inner wrapper. The title bar is
		     the space above the content, so the top padding is small. -->
		<div class="min-h-0 flex-1">
			<main bind:this={main} {@attach overlayScrollbars}>
				<!-- In a wide window, the content is a column at the center. -->
				<div class="px-8 pt-2 pb-8">
					<div class="mx-auto max-w-4xl">{@render children()}</div>
				</div>
			</main>
		</div>
	</div>
</div>

<style>
	/* Only the content moves. The sidebar is not captured, so it stays live and its highlight moves. */
	:global(:root) {
		view-transition-name: none;
	}
	main {
		view-transition-name: settings-content;
	}

	/* The direction from which the new content comes. The old content leaves the oposite way. */
	:global(:root:active-view-transition-type(forward)) {
		--enter-x: 2.5rem;
	}
	:global(:root:active-view-transition-type(back)) {
		--enter-x: -2.5rem;
	}
	:global(:root:active-view-transition-type(down)) {
		--enter-y: 1.5rem;
	}
	:global(:root:active-view-transition-type(up)) {
		--enter-y: -1.5rem;
	}

	/* Page grids (see GridPreview) with no match on the other side move with the content. */
	:global(::view-transition-old(settings-content)),
	:global(::view-transition-old(.page-grid):only-child) {
		animation: 150ms cubic-bezier(0.2, 0, 0, 1) both settings-leave;
	}
	:global(::view-transition-new(settings-content)),
	:global(::view-transition-new(.page-grid):only-child) {
		animation: 350ms cubic-bezier(0.05, 0.7, 0.1, 1) 75ms both settings-enter;
	}

	/* The page that opens or closes. Its grid grows into the editor, or shrinks into its card. */
	:global(::view-transition-group(.page-grid)) {
		animation-duration: 400ms;
		animation-timing-function: cubic-bezier(0.2, 0, 0, 1);
	}

	@keyframes -global-settings-leave {
		to {
			opacity: 0;
			translate: calc(-1 * var(--enter-x, 0px)) calc(-1 * var(--enter-y, 0px));
		}
	}
	@keyframes -global-settings-enter {
		from {
			opacity: 0;
			translate: var(--enter-x, 0px) var(--enter-y, 0px);
		}
	}
</style>
