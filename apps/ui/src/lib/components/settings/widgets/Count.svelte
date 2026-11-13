<script lang="ts">
	let { count, broken = 0 }: { count: number; broken?: number } = $props();
</script>

<!-- A count in a pill. It is red when the host cannot read `broken` of the widgets. -->
<span
	class={[
		'min-w-5 rounded-full px-1.5 text-center text-xs leading-5 tabular-nums',
		broken ? 'bg-destructive/15 text-destructive' : 'bg-foreground/10 text-muted-foreground'
	]}
	title={broken ? `${broken} can't be read` : undefined}
>
	{count}
</span>
