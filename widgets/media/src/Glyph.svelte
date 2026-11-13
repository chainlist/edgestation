<script lang="ts">
	import { ICONS } from './icons.ts';

	let { name }: { name: keyof typeof ICONS } = $props();
</script>

<svg viewBox="0 0 24 24" aria-hidden="true"><path d={ICONS[name]} /></svg>
