<script lang="ts" module>
	/**
	 * Saves the settings field that the user still types in. Text fields save on blur, and a close
	 * of the settings does not cause a blur. Call it before the settings close or show another
	 * widget.
	 */
	export function commitSettings() {
		const field = document.activeElement;
		if (field instanceof HTMLElement && field.closest('[data-widget-settings]')) field.blur();
	}
</script>

<script lang="ts">
	import { Input } from '#lib/components/ui/input/index.ts';
	import { Label } from '#lib/components/ui/label/index.ts';
	import * as Sheet from '#lib/components/ui/sheet/index.ts';
	import { setPlacementProfile, setPlacementSetting } from '#lib/layout/pages.ts';
	import type { Page, Placement } from '#lib/layout/types.ts';
	import { overlayScrollbars } from '#lib/scrollbars.ts';
	import { editPage } from '#lib/state/page-actions.ts';
	import { profiles } from '#lib/state/settings.svelte.ts';
	import { installed } from '#lib/widgets/installed.svelte.ts';
	import { usesWebview } from '#lib/widgets/permissions.ts';
	import { resolveSettings, visibleSettings } from '#lib/widgets/settings.ts';
	import { readSource, sourcedChoices } from '#lib/widgets/sources.ts';
	import type { SelectSetting } from '#lib/widgets/types.ts';
	import ChoiceSelect from '#lib/components/settings/controls/ChoiceSelect.svelte';

	/**
	 * The settings of `item`, in a sheet that comes from the right of the window. The sheet is
	 * closed while `item` is not set. `onclose` must call `commitSettings` first.
	 */
	let { page, item, onclose }: { page: Page; item?: Placement; onclose: () => void } = $props();

	/** The choice value of the Default profile, which has no id. Profile ids are hex. */
	const DEFAULT = 'default';

	let content = $state<HTMLElement | null>(null);

	// The last placement that showed. The sheet keeps its content wile it slides out.
	let last: Placement | undefined;
	const current = $derived.by(() => (item ? (last = item) : last));

	const widget = $derived(current && installed.current.find((w) => w.id === current.widget));
	const schema = $derived(widget?.settings ?? []);
	const values = $derived(resolveSettings(schema, current?.settings));
	const shown = $derived(visibleSettings(schema, values));
	const profileChoices = $derived([
		{ value: DEFAULT, label: 'Default' },
		...profiles.current.map((p) => ({ value: p.id, label: p.name }))
	]);

	/** The choices that each select with a `source` reads, by key. The sheet reads them on open. */
	let sourced = $state.raw<Record<string, SelectSetting['options']>>({});
	const sourcedKeys = $derived(
		schema.flatMap((setting) =>
			setting.type === 'select' && setting.source ? [[setting.key, setting.source] as const] : []
		)
	);
	// Use the id of the placement, not its object. The object changes each time an setting is saved.
	const shownId = $derived(item?.id);
	$effect(() => {
		sourced = {};
		if (!shownId) return;
		let open = true;
		for (const [key, source] of sourcedKeys) {
			readSource(source).then((choices) => {
				if (open) sourced = { ...sourced, [key]: choices };
			});
		}
		return () => (open = false);
	});
</script>

<Sheet.Root bind:open={() => !!item, (open) => open || onclose()}>
	<Sheet.Content
		bind:ref={content}
		data-widget-settings
		side="right"
		class="gap-0 outline-none"
		onOpenAutoFocus={(event) => {
			// Focus the sheet, not its first field. The field would show a focus ring after a click.
			// Tab still goes to the fields from the sheet.
			event.preventDefault();
			content?.focus({ preventScroll: true });
		}}
		onCloseAutoFocus={(event) => event.preventDefault()}
	>
		{#if current}
			<!-- Space on the right for the close button. -->
			<Sheet.Header class="pr-12">
				<Sheet.Title>{widget?.name ?? current.widget}</Sheet.Title>
				{#if widget?.description}
					<Sheet.Description>{widget.description}</Sheet.Description>
				{/if}
			</Sheet.Header>
			<!-- Holds the overlay scrollbars of the fields, which fill it (see `overlayScrollbars`). -->
			<div class="min-h-0 flex-1">
				<div {@attach overlayScrollbars}>
					<div class="grid gap-5 px-4 pt-2 pb-4">
						{#if widget && usesWebview(widget)}
							<div class="grid gap-2">
								<Label>Profile</Label>
								<ChoiceSelect
									class="w-full"
									choices={profileChoices}
									value={current.profile ?? DEFAULT}
									onchange={(value) =>
										editPage(page.id, (p) =>
											setPlacementProfile(p, current.id, value === DEFAULT ? undefined : value)
										)}
								/>
							</div>
						{/if}
						{#each shown as setting (setting.key)}
							<div class="grid gap-2">
								<Label>{setting.label}</Label>
								{#if setting.type === 'text'}
									<!-- The value saves on Enter or on blur, not on each key, because each
									     change mounts the widget again. -->
									<Input
										type={setting.secret ? 'password' : 'text'}
										value={values[setting.key]}
										placeholder={setting.placeholder}
										onchange={(event) => {
											const { value } = event.currentTarget;
											editPage(page.id, (p) =>
												setPlacementSetting(p, current.id, setting.key, value)
											);
										}}
									/>
								{:else}
									<ChoiceSelect
										class="w-full"
										choices={setting.source
											? sourcedChoices(setting, sourced[setting.key], values[setting.key])
											: setting.options}
										value={values[setting.key]}
										onchange={(value) =>
											editPage(page.id, (p) =>
												setPlacementSetting(p, current.id, setting.key, value)
											)}
									/>
								{/if}
							</div>
						{/each}
					</div>
				</div>
			</div>
		{/if}
	</Sheet.Content>
</Sheet.Root>
