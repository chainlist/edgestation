<script lang="ts">
	import type { Snippet } from 'svelte';
	import * as Card from '#lib/components/ui/card/index.ts';
	import SettingsSection from './SettingsSection.svelte';

	let { title, note, children }: { title: string; note?: Snippet; children: Snippet } = $props();
</script>

<!-- A card with a title and a stack of settings, one row for each (see SettingsRow). An optional
     note goes below the card. -->
<SettingsSection {title}>
	<Card.Root class="gap-0 divide-y py-0">{@render children()}</Card.Root>
	{#if note}
		<p class="px-1 text-sm text-muted-foreground">{@render note()}</p>
	{/if}
</SettingsSection>
