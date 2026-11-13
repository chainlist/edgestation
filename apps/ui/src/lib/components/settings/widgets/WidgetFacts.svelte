<script lang="ts">
	import BadgeCheck from '@lucide/svelte/icons/badge-check';
	import type { Fact } from '#lib/widgets/marketplace/listing.ts';

	let { facts }: { facts: Fact[] } = $props();
</script>

{#each facts as fact (fact.text)}
	<span
		class={['inline-flex items-center gap-1', fact.mono && 'font-mono', fact.title && 'relative']}
		title={fact.title}
	>
		{#if fact.icon}
			<fact.icon class="size-3.5" />
		{/if}
		{fact.text}
		{#if fact.verified}
			<span class="relative text-primary" title="Verified">
				<BadgeCheck class="size-3.5" aria-label="Verified" />
			</span>
		{/if}
	</span>
{/each}
