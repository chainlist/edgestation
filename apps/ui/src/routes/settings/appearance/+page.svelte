<script lang="ts">
	import Check from '@lucide/svelte/icons/check';
	import ImagePlus from '@lucide/svelte/icons/image-plus';
	import Moon from '@lucide/svelte/icons/moon';
	import Sun from '@lucide/svelte/icons/sun';
	import Trash from '@lucide/svelte/icons/trash';
	import EdgePreview from '#lib/components/settings/EdgePreview.svelte';
	import PageHeader from '#lib/components/settings/PageHeader.svelte';
	import SegmentedControl from '#lib/components/settings/controls/SegmentedControl.svelte';
	import SettingsGroup from '#lib/components/settings/SettingsGroup.svelte';
	import SettingsRow from '#lib/components/settings/SettingsRow.svelte';
	import SliderInput from '#lib/components/settings/controls/SliderInput.svelte';
	import { Button } from '#lib/components/ui/button/index.ts';
	import { XENEON_EDGE } from '#lib/layout/grid.ts';
	import { patch } from '#lib/state/persisted.svelte.ts';
	import {
		backgroundBlur,
		backgroundImage,
		pages,
		theme,
		widgetBlur,
		widgetOpacity
	} from '#lib/state/settings.svelte.ts';
	import { edgeMonitorSize } from '#lib/tauri/edge-window.ts';
	import { readBackground } from '#lib/theme/background.ts';
	import { ACCENTS } from '#lib/theme/presets.ts';
	import type { ThemeSettings } from '#lib/theme/types.ts';

	const MODES = [
		{ mode: 'dark', label: 'Dark', icon: Moon },
		{ mode: 'light', label: 'Light', icon: Sun }
	] as const;

	const id = $props.id();
	const set = (changes: Partial<ThemeSettings>) => patch(theme, changes);

	let picker: HTMLInputElement;
	let reading = $state(false);
	let error = $state<string>();

	async function pick() {
		const file = picker.files?.[0];
		// Clear the value, so the same file again still counts as a change.
		picker.value = '';
		if (!file) return;
		reading = true;
		error = undefined;
		// Size the image for the monitor of the Edge, or for a Xeneon Edge if the monitor is not found.
		const screen = await edgeMonitorSize().catch((e: unknown) => void console.error(e));
		try {
			backgroundImage.current = await readBackground(file, screen ?? XENEON_EDGE);
		} catch (e) {
			console.error(e);
			error = `Could not read ${file.name} as an image.`;
		} finally {
			reading = false;
		}
	}
</script>

{#snippet cssVar(name: string)}
	<code class="rounded bg-muted px-1 font-mono text-xs whitespace-nowrap text-foreground"
		>var({name})</code
	>
{/snippet}

<PageHeader
	title="Appearance"
	description="Colors and corners of the Edge and this window, and the image behind the Edge's pages."
/>

<div class="grid gap-8">
	<SettingsGroup title="Theme">
		<SettingsRow label="Mode" description="Light or dark surfaces and text.">
			<SegmentedControl
				name="Mode"
				items={MODES}
				selected={(m) => m.mode === theme.current.mode}
				onpick={(m) => set({ mode: m.mode })}
			>
				{#snippet label(m)}<m.icon />{m.label}{/snippet}
			</SegmentedControl>
		</SettingsRow>
		<SettingsRow
			label="Corner radius"
			description="Of widgets, cards and buttons."
			for="{id}-radius"
		>
			<SliderInput
				id="{id}-radius"
				value={theme.current.radius}
				min={0}
				max={1.5}
				step={0.125}
				format={(v) => `${v}rem`}
				onchange={(radius) => set({ radius })}
			/>
		</SettingsRow>
	</SettingsGroup>

	<SettingsGroup title="Accent color">
		<SettingsRow label="Preset" description="A starting point; fine-tune it below.">
			<div role="group" aria-label="Accent presets" class="flex flex-wrap gap-2.5">
				{#each ACCENTS as accent (accent.name)}
					{@const on = accent.hue === theme.current.hue && accent.chroma === theme.current.chroma}
					<button
						type="button"
						title={accent.name}
						aria-label={accent.name}
						aria-pressed={on}
						class={[
							'grid size-7 place-items-center rounded-full ring-offset-2 ring-offset-card transition-shadow outline-none focus-visible:ring-3 focus-visible:ring-ring/50',
							on ? 'ring-2 ring-foreground/70' : 'hover:ring-2 hover:ring-foreground/20'
						]}
						style:background="oklch(0.7 {accent.chroma}
						{accent.hue})"
						onclick={() => set({ hue: accent.hue, chroma: accent.chroma })}
					>
						{#if on}<Check class="size-4 text-black/70" />{/if}
					</button>
				{/each}
			</div>
		</SettingsRow>
		<SettingsRow label="Hue" description="Position on the color wheel." for="{id}-hue">
			<SliderInput
				id="{id}-hue"
				value={theme.current.hue}
				min={0}
				max={360}
				step={1}
				format={(v) => `${v}°`}
				onchange={(hue) => set({ hue })}
			/>
		</SettingsRow>
		<SettingsRow label="Saturation" description="From gray at 0 to vivid." for="{id}-saturation">
			<SliderInput
				id="{id}-saturation"
				value={theme.current.chroma}
				min={0}
				max={0.3}
				step={0.01}
				format={(v) => v.toFixed(2)}
				onchange={(chroma) => set({ chroma })}
			/>
		</SettingsRow>
		{#snippet note()}
			Widgets can use the same colors through CSS variables, such as
			{@render cssVar('--primary')}, {@render cssVar('--card')} and
			{@render cssVar('--muted-foreground')}.
		{/snippet}
	</SettingsGroup>

	<!-- The widget settings apply only over an image, so they show only when there is an image. -->
	<SettingsGroup title="Edge background">
		<SettingsRow
			label="Image"
			description="Behind the Edge's pages, still while they slide."
			{error}
		>
			{#if backgroundImage.current}
				<!-- A small copy of the Edge with the widgets of the first page. It shows the result of
				     all the settings below. A click on it selcts another image. -->
				<button
					type="button"
					title="Change image"
					aria-label="Change image"
					disabled={reading}
					class="group relative rounded-md outline-none focus-visible:ring-3 focus-visible:ring-ring/50"
					onclick={() => picker.click()}
				>
					<EdgePreview
						image={backgroundImage.current}
						blur={backgroundBlur.current}
						opacity={widgetOpacity.current}
						widgetBlur={widgetBlur.current}
						radius={theme.current.radius}
						page={pages.current[0]}
					/>
					<span
						class={[
							'absolute inset-0 flex items-center justify-center gap-1.5 rounded-md bg-black/50 text-xs font-medium text-white transition-opacity',
							!reading && 'opacity-0 group-hover:opacity-100 group-focus-visible:opacity-100'
						]}
					>
						<ImagePlus class="size-4" />{reading ? 'Reading…' : 'Change'}
					</span>
				</button>
				<Button
					variant="ghost"
					size="icon"
					title="Remove image"
					aria-label="Remove image"
					onclick={() => (backgroundImage.current = null)}
				>
					<Trash />
				</Button>
			{:else}
				<Button variant="outline" disabled={reading} onclick={() => picker.click()}>
					<ImagePlus />{reading ? 'Reading…' : 'Choose image'}
				</Button>
			{/if}
			<input bind:this={picker} type="file" accept="image/*" hidden onchange={pick} />
		</SettingsRow>
		{#if backgroundImage.current}
			<SettingsRow label="Image blur" for="{id}-blur">
				<SliderInput
					id="{id}-blur"
					value={backgroundBlur.current}
					min={0}
					max={1}
					step={0.05}
					format={(v) => v.toFixed(2)}
					onchange={(blur) => (backgroundBlur.current = blur)}
				/>
			</SettingsRow>
			<SettingsRow
				label="Widget opacity"
				description="Below 1, the image shows through the widgets."
				for="{id}-widget-opacity"
			>
				<SliderInput
					id="{id}-widget-opacity"
					value={widgetOpacity.current}
					min={0}
					max={1}
					step={0.05}
					format={(v) => v.toFixed(2)}
					onchange={(opacity) => (widgetOpacity.current = opacity)}
				/>
			</SettingsRow>
			<SettingsRow
				label="Widget blur"
				description="Frosts the image seen through the widgets."
				for="{id}-widget-blur"
			>
				<SliderInput
					id="{id}-widget-blur"
					value={widgetBlur.current}
					min={0}
					max={1}
					step={0.05}
					format={(v) => v.toFixed(2)}
					disabled={widgetOpacity.current >= 1}
					onchange={(blur) => (widgetBlur.current = blur)}
				/>
			</SettingsRow>
		{/if}
		{#snippet note()}
			{#if backgroundImage.current}
				Widgets showing a web page, such as URL and YouTube, stay opaque.
			{:else}
				With an image, the widgets can also show it through them, frosted.
			{/if}
		{/snippet}
	</SettingsGroup>
</div>
