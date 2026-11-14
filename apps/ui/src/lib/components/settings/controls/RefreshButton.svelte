<script lang="ts">
	import RefreshCw from '@lucide/svelte/icons/refresh-cw';
	import { Button } from '#lib/components/ui/button/index.ts';

	let {
		label,
		busy,
		onclick
	}: {
		/** The tooltip, which screen readers also read as the name of the button. */
		label: string;
		/** The arrows turn and the button is off while this is true. */
		busy: boolean;
		onclick: () => void;
	} = $props();
</script>

<Button variant="ghost" size="icon" title={label} aria-label={label} disabled={busy} {onclick}>
	<RefreshCw class={busy ? 'animate-spin' : undefined} />
</Button>
