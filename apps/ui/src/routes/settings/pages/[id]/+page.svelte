<script lang="ts">
	import ArrowLeft from '@lucide/svelte/icons/arrow-left';
	import { page as route } from '$app/state';
	import AddWidget from '#lib/components/settings/pages/AddWidget.svelte';
	import GridEditor from '#lib/components/settings/pages/GridEditor.svelte';
	import PageForm from '#lib/components/settings/pages/PageForm.svelte';
	import PageHeader from '#lib/components/settings/PageHeader.svelte';
	import WidgetSettings, {
		commitSettings
	} from '#lib/components/settings/pages/WidgetSettings.svelte';
	import { Button } from '#lib/components/ui/button/index.ts';
	import { pages } from '#lib/state/settings.svelte.ts';
	import { shownPage } from '#lib/state/shown-page.svelte.ts';
	import { installed } from '#lib/widgets/installed.svelte.ts';
	import { hasSettings } from '#lib/widgets/settings.ts';
	import type { PageProps } from './$types';

	let { params }: PageProps = $props();

	const page = $derived(pages.current.find((p) => p.id === params.id));

	// Back on the page list, the tab of this page is open, and its grid shrinks back into it.
	$effect.pre(() => {
		shownPage.id = params.id;
	});

	// `?widget=<placement id>` opens the settings of that widget, for example from the Browser
	// profiles page.
	let selectedId = $state(route.url.searchParams.get('widget') ?? undefined);
	// The widget whose settings are open. It is undefined if the widget has no settings or is
	// remved, or if another page opens.
	const editing = $derived.by(() => {
		const item = page?.items.find((item) => item.id === selectedId);
		const widget = item && installed.current.find((w) => w.id === item.widget);
		return hasSettings(widget, item?.settings) ? item : undefined;
	});

	/** A click on a widget opens its settings, or closes them if they are already open. */
	function toggle(id: string) {
		commitSettings();
		selectedId = editing?.id === id ? undefined : id;
	}

	function close() {
		commitSettings();
		selectedId = undefined;
	}
</script>

{#if page}
	<header class="mb-4 flex items-center gap-2">
		<Button
			variant="ghost"
			size="icon"
			href="/settings/pages"
			aria-label="All pages"
			title="All pages"
		>
			<ArrowLeft />
		</Button>
		<PageForm {page} />
	</header>
	<div class="grid gap-3">
		<AddWidget {page} />
		<GridEditor {page} selected={editing?.id} onselect={toggle} ondragstart={close} />
		<p class="text-xs text-muted-foreground">
			Changes show on the Edge right away. Click a widget to change its settings, when it has some.
			Drag it to move it, drag any of its corners to resize it. Hover it to remove it. A red widget
			does not fit yet. It is not saved and does not show on the Edge until you move or resize it
			onto free cells.
		</p>
		<WidgetSettings {page} item={editing} onclose={close} />
	</div>
{:else}
	<PageHeader title="Page not found">
		{#snippet actions()}
			<Button variant="outline" href="/settings/pages"><ArrowLeft />All pages</Button>
		{/snippet}
	</PageHeader>
{/if}
