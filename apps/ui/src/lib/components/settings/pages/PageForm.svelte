<script lang="ts">
	import { MAX_COLS, MAX_ROWS } from '#lib/layout/grid.ts';
	import { resizePage } from '#lib/layout/pages.ts';
	import type { Page } from '#lib/layout/types.ts';
	import { editPage } from '#lib/state/page-actions.ts';
	import NumberField from '#lib/components/settings/controls/NumberField.svelte';

	let { page }: { page: Page } = $props();

	const edit = (fn: (page: Page) => Page) => editPage(page.id, fn);
</script>

<!-- The title row of the editor, with the page name and the grid size. The user edits the name in
     place. -->
<div class="flex min-w-0 flex-1 flex-wrap items-center justify-between gap-x-6 gap-y-2">
	<!-- It looks like a heading until the user hovers over it or focuses it. -->
	<input
		aria-label="Page name"
		title="Rename"
		value={page.name}
		class="field-sizing-content h-9 max-w-full min-w-24 rounded-md border border-transparent bg-transparent px-2 text-xl font-semibold transition-[color,box-shadow,border-color] outline-none hover:border-input focus-visible:border-ring focus-visible:ring-3 focus-visible:ring-ring/50"
		onchange={(event) => {
			const name = event.currentTarget.value.trim();
			if (name) edit((p) => ({ ...p, name }));
			else event.currentTarget.value = page.name;
		}}
	/>
	<div class="flex items-center gap-2 text-sm text-muted-foreground">
		Grid
		<NumberField
			label="Columns"
			value={page.cols}
			min={1}
			max={MAX_COLS}
			onchange={(cols) => edit((p) => resizePage(p, cols, p.rows))}
		/>
		×
		<NumberField
			label="Rows"
			value={page.rows}
			min={1}
			max={MAX_ROWS}
			onchange={(rows) => edit((p) => resizePage(p, p.cols, rows))}
		/>
	</div>
</div>
