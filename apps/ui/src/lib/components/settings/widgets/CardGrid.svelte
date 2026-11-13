<script lang="ts">
	import type { Snippet } from 'svelte';

	let { children }: { children: Snippet } = $props();
</script>

<!-- Two columns when the content is at least 36rem wide. -->
<div class="@container">
	<div class="grid gap-3 @xl:grid-cols-2">{@render children()}</div>
</div>
