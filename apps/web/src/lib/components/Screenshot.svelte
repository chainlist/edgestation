<script lang="ts">
	let {
		src,
		alt,
		width = 1100,
		height = 720
	}: { src: string; alt: string; width?: number; height?: number } = $props();
</script>

<!-- A capture of the Settings window: 1100 × 720, its own title bar included, if it is not
     cropped. -->
<img
	{src}
	{alt}
	{width}
	{height}
	loading="lazy"
	decoding="async"
	class="w-full rounded-xl border border-white/10 shadow-2xl ring-1 shadow-black/60 ring-black"
/>
