<script lang="ts">
	import Layers from '@lucide/svelte/icons/layers';
	import Monitor from '@lucide/svelte/icons/monitor';
	import Pointer from '@lucide/svelte/icons/pointer';

	const basics = [
		{
			icon: Monitor,
			title: 'Always on, on the Edge',
			text: 'Full screen on your secondary display, with swipeable pages of widgets.'
		},
		{
			icon: Layers,
			title: 'Set up from the tray',
			text: 'Click the tray icon to open Settings and arrange your pages.'
		},
		{
			icon: Pointer,
			title: 'Touch, mouse or keys',
			text: 'Swipe between pages with two fingers, scroll over the page dots, or use global shortcuts.'
		}
	];
</script>

<section class="border-y bg-card/30">
	<div class="mx-auto grid max-w-6xl gap-8 px-4 py-14 sm:grid-cols-3 sm:px-6">
		{#each basics as { icon: Icon, title, text } (title)}
			<div>
				<div class="grid size-10 place-items-center rounded-lg bg-primary/15 text-primary">
					<Icon class="size-5" />
				</div>
				<h2 class="mt-4 font-semibold">{title}</h2>
				<p class="mt-1.5 text-sm text-muted-foreground">{text}</p>
			</div>
		{/each}
	</div>
</section>
