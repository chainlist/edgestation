<script lang="ts">
	import type Store from '@lucide/svelte/icons/store';
	import type { Snippet } from 'svelte';

	let {
		icon: Icon,
		title,
		text,
		children
	}: { icon: typeof Store; title: string; text: string; children?: Snippet } = $props();
</script>

<!-- Fills an empty card. `children` is an optional action below the text. -->
<div class="grid justify-items-center gap-3 px-6 py-10 text-center">
	<span class="flex size-10 items-center justify-center rounded-lg bg-muted text-muted-foreground">
		<Icon class="size-5" />
	</span>
	<div class="grid gap-1">
		<p class="font-medium">{title}</p>
		<p class="max-w-sm text-muted-foreground">{text}</p>
	</div>
	{@render children?.()}
</div>
