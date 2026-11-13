<script lang="ts">
	import { blurRadius } from '#lib/theme/background.ts';

	/** `blur` is 0 (sharp) to 1. The image fills its closest positioned ancestor, which clips it. */
	let { image, blur }: { image: string; blur: number } = $props();

	const radius = $derived(blurRadius(blur));
</script>

<!-- The image goes past the box by two blur radii, so the blurred edges never fade into the color
     below it. The radius is in cqmin (see background.ts). -->
<div class="pointer-events-none absolute" style:inset="{-2 * radius}cqmin" aria-hidden="true">
	<img
		src={image}
		alt=""
		decoding="async"
		draggable="false"
		class="size-full object-cover"
		style:filter={radius > 0 ? `blur(${radius}cqmin)` : undefined}
	/>
</div>
