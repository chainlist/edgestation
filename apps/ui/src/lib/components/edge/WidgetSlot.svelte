<script lang="ts">
	import { cellStyle } from '#lib/layout/grid.ts';
	import type { Placement } from '#lib/layout/types.ts';
	import type { Pause } from '#lib/widgets/api/timers.ts';
	import { widgetRevision } from '#lib/widgets/revisions.svelte.ts';
	import WidgetError from './WidgetError.svelte';
	import WidgetMount from './WidgetMount.svelte';

	let { item, pause, started }: { item: Placement; pause: Pause; started: boolean } = $props();
</script>

<!-- contain-strict keeps the layout and paint work of a widget inside its cell. The settings of the
     Edge page set the card opacity and blur (see widgetGlassVars). Without them, the card is
     opaque. -->
<div
	class="overflow-hidden rounded-xl border bg-card/[var(--widget-opacity,100%)] text-card-foreground [backdrop-filter:var(--widget-backdrop,none)] contain-strict"
	style={cellStyle(item)}
>
	<!-- The card shows at once, but the widget mounts only once its page has shown (`started`).
	     When the marketplace replaces or removes the widget files, the widget mounts again from
	     them. This is also true from it's error card. -->
	{#if started}
		{#key widgetRevision(item.widget)}
			<svelte:boundary>
				<WidgetMount
					widget={item.widget}
					instance={item.id}
					settings={item.settings}
					profile={item.profile}
					{pause}
				/>
				{#snippet failed(error, reset)}
					<WidgetError name={item.widget} {error} {reset} />
				{/snippet}
			</svelte:boundary>
		{/key}
	{/if}
</div>
