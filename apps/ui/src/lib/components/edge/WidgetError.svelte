<script lang="ts">
	import { Button } from '#lib/components/ui/button/index.ts';

	let { name, error, reset }: { name: string; error: unknown; reset: () => void } = $props();

	// SvelteKit turns boundary errors into App.Error (see hooks.client.ts).
	const message = $derived((error as App.Error).message);
</script>

<div class="flex size-full flex-col items-center justify-center gap-2 p-4 text-center">
	<p class="text-sm font-medium text-destructive">{name} stopped working</p>
	<p class="line-clamp-3 text-xs break-all text-muted-foreground">{message}</p>
	<Button size="sm" variant="outline" onclick={reset}>Retry</Button>
</div>
