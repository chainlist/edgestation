<script lang="ts" module>
	import Keyboard from '@lucide/svelte/icons/keyboard';
	import LayoutGrid from '@lucide/svelte/icons/layout-grid';
	import Monitor from '@lucide/svelte/icons/monitor';
	import Palette from '@lucide/svelte/icons/palette';
	import Puzzle from '@lucide/svelte/icons/puzzle';
	import Settings from '@lucide/svelte/icons/settings';
	import Users from '@lucide/svelte/icons/users';

	/** The sidebar groups. Browser profiles are in Dashboard because web view widgets use them. */
	const GROUPS = [
		{
			label: 'App',
			sections: [
				{ href: '/settings/general', label: 'General', icon: Settings },
				{ href: '/settings/shortcuts', label: 'Shortcuts', icon: Keyboard }
			]
		},
		{
			label: 'Dashboard',
			sections: [
				{ href: '/settings/pages', label: 'Pages', icon: LayoutGrid },
				{ href: '/settings/widgets', label: 'Widgets', icon: Puzzle },
				{ href: '/settings/profiles', label: 'Browser profiles', icon: Users }
			]
		},
		{
			label: 'Screen',
			sections: [
				{ href: '/settings/display', label: 'Display', icon: Monitor },
				{ href: '/settings/appearance', label: 'Appearance', icon: Palette }
			]
		}
	];

	/** The settings sections, in sidebar order. */
	export const SECTIONS = GROUPS.flatMap((group) => group.sections);
</script>

<script lang="ts">
	import RotateCcw from '@lucide/svelte/icons/rotate-ccw';
	import { page } from '$app/state';
	import { Button } from '#lib/components/ui/button/index.ts';
	import { reloadEdge } from '#lib/tauri/events.ts';

	// Each starts as `null`, the fallback value of Button. A bind to `undefined` throws an error.
	const links: Record<string, HTMLElement | null> = $state(
		Object.fromEntries(SECTIONS.map(({ href }) => [href, null]))
	);
	const active = $derived(SECTIONS.find(({ href }) => page.url.pathname.startsWith(href)));
	const highlighted = $derived(active && links[active.href]);
</script>

<nav class="flex w-52 shrink-0 flex-col gap-1 border-r bg-sidebar px-3 pb-3">
	<!-- The left part of the title bar (see TitleBar), as high as it, and a drag region too. -->
	<p
		data-tauri-drag-region
		class="-mx-3 mb-2 flex h-10 shrink-0 items-center px-5.5 text-sm font-semibold select-none"
	>
		EdgeStation
	</p>
	<div class="relative grid gap-4">
		{#if highlighted}
			<!-- The higlight of the current section, in the accent color of the theme. It slides
			     from link to link, also across groups. -->
			<span
				class="absolute inset-x-0 top-0 h-9 rounded-md bg-primary/15 transition-transform duration-150 ease-out motion-reduce:transition-none"
				style:translate="0 {highlighted.offsetTop}px"
			></span>
		{/if}
		{#each GROUPS as group (group.label)}
			<div class="grid gap-1">
				<p class="px-2.5 pb-1 text-xs font-medium text-muted-foreground">{group.label}</p>
				{#each group.sections as { href, label, icon: Icon } (href)}
					<!-- The hover is a lighter tint of the accent. The current section has the
					     highlight below it, so it gets no hover of its own. -->
					<Button
						bind:ref={links[href]}
						{href}
						variant="ghost"
						aria-current={href === active?.href ? 'page' : undefined}
						class={[
							'relative justify-start',
							href === active?.href
								? 'text-primary hover:bg-transparent hover:text-primary dark:hover:bg-transparent'
								: 'hover:bg-primary/10 dark:hover:bg-primary/10'
						]}
					>
						<Icon />{label}
					</Button>
				{/each}
			</div>
		{/each}
	</div>
	<footer class="mt-auto grid">
		<Button
			variant="ghost"
			onclick={reloadEdge}
			class="justify-start hover:bg-primary/10 dark:hover:bg-primary/10"
		>
			<RotateCcw />Reload Edge
		</Button>
	</footer>
</nav>
