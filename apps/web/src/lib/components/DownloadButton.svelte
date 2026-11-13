<script lang="ts">
	import Download from '@lucide/svelte/icons/download';
	import type { Snippet } from 'svelte';

	let {
		href,
		size = 'md',
		variant = 'primary',
		children
	}: {
		href: string;
		size?: 'md' | 'lg';
		/** The button of the platform of the visitor is primary. The others are secondary. */
		variant?: 'primary' | 'secondary';
		children: Snippet;
	} = $props();
</script>

<a
	{href}
	class={[
		'inline-flex items-center justify-center gap-2 rounded-xl font-semibold transition',
		variant === 'primary'
			? 'bg-primary text-primary-foreground shadow-[0_8px_30px_-8px] shadow-primary/60 hover:brightness-110'
			: 'border bg-card/60 hover:bg-muted',
		size === 'lg' ? 'h-14 px-8 text-lg' : 'h-11 px-5'
	]}
>
	<Download class={size === 'lg' ? 'size-5' : 'size-4'} />{@render children()}
</a>
