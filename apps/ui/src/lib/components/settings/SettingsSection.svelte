<script lang="ts">
	import type { Snippet } from 'svelte';

	let { title, children }: { title: string; children: Snippet } = $props();
</script>

<section class="grid gap-3">
	<h2 class="text-sm font-medium">{title}</h2>
	{@render children()}
</section>
