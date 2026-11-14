<script lang="ts">
	import Plus from '@lucide/svelte/icons/plus';
	import { Button } from '#lib/components/ui/button/index.ts';
	import { widgetSize } from '#lib/layout/defaults.ts';
	import { newPlacement } from '#lib/layout/pages.ts';
	import type { Page } from '#lib/layout/types.ts';
	import { editWaiting } from '#lib/state/page-actions.ts';
	import { installed } from '#lib/widgets/installed.svelte.ts';
	import ChoiceSelect from '#lib/components/settings/controls/ChoiceSelect.svelte';

	let { page }: { page: Page } = $props();

	let choice = $state('');

	// The label shows the size (columns×rows), so the user can see if the widget fits on the page.
	const choices = $derived(
		installed.current
			.filter((widget) => !widget.error)
			.map((widget) => {
				const { w, h } = widgetSize(widget);
				return { value: widget.id, label: `${widget.name} (${w}×${h})` };
			})
	);

	// The new widget waits, and `editWaiting` saves it at once if it fits. If not, it stays red on
	// the grid until the user moves or resizes it onto free cells.
	function add() {
		const widget = installed.current.find((w) => w.id === choice);
		if (!widget) return;
		const item = newPlacement(page, widget.id, widgetSize(widget));
		editWaiting(page.id, (items) => [...items, item]);
	}
</script>

<div class="flex flex-wrap items-center gap-2">
	<ChoiceSelect
		{choices}
		value={choice}
		placeholder="Choose a widget"
		onchange={(v) => (choice = v)}
	/>
	<Button disabled={!choice} onclick={add}><Plus />Add</Button>
</div>
