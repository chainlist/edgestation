<script lang="ts">
	let { class: className }: { class?: string } = $props();
</script>

<span
	class={[
		'inline-flex items-center rounded-full border border-primary/30 bg-primary/10 px-2 py-0.5 text-[11px] leading-none font-semibold tracking-wide text-primary uppercase',
		className
	]}
>
	Beta
</span>
