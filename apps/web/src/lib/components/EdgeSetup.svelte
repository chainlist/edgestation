<script lang="ts">
	let {
		edge,
		edgeAlt,
		monitor,
		monitorAlt,
		side = 'right',
		eager = false
	}: {
		/** A 2560 × 720 capture of the Edge. */
		edge: string;
		edgeAlt: string;
		/** A 1920 × 1080 capture of EdgeStation on a normal monitor. */
		monitor: string;
		monitorAlt: string;
		/** The side of the monitor. The Edge is in front of it, a bit to the other side. */
		side?: 'left' | 'right';
		/** If true, the images load first, as for the top of the page. */
		eager?: boolean;
	} = $props();

	const loading = $derived(eager ? 'eager' : 'lazy');
	const fetchpriority = $derived(eager ? 'high' : 'auto');
</script>

<!-- A desk seen from the front: a normal monitor at the back, and the Edge in front of its lower
     part. The top padding is a percent of the width, so the overlap stays the same at all sizes. -->
<div class="relative pt-[20%]">
	<div class={['absolute top-0 w-[74%]', side === 'right' ? 'right-0' : 'left-0']}>
		<div class="rounded-[0.8rem] bg-gradient-to-b from-zinc-700 to-zinc-900 p-[2px] sm:p-[3px]">
			<div class="rounded-[0.7rem] bg-black p-1 pb-2 sm:p-1.5 sm:pb-3">
				<img
					src={monitor}
					width="1920"
					height="1080"
					alt={monitorAlt}
					class="w-full rounded-sm brightness-[0.85]"
					{loading}
					{fetchpriority}
					decoding="async"
				/>
			</div>
		</div>
	</div>

	<div class={['relative w-[88%]', side === 'right' ? 'mr-auto' : 'ml-auto']}>
		<div
			class="rounded-[1.1rem] bg-gradient-to-b from-zinc-700 to-zinc-900 p-[3px] shadow-[0_30px_80px_-20px] shadow-black"
		>
			<div class="rounded-[1rem] bg-black p-1 sm:p-2">
				<img
					src={edge}
					width="2560"
					height="720"
					alt={edgeAlt}
					class="w-full rounded-md"
					{loading}
					{fetchpriority}
				/>
			</div>
		</div>
	</div>
</div>
