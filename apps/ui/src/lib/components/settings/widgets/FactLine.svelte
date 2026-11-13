<script lang="ts">
	import type { Snippet } from 'svelte';

	let { class: className, children }: { class?: string; children: Snippet } = $props();
</script>

<!-- Small facts in a line that wraps. Icons keep the facts appart, so no separator starts a
     line. -->
<div
	class={['flex flex-wrap items-center gap-x-3 gap-y-1 text-xs text-muted-foreground', className]}
>
	{@render children()}
</div>
