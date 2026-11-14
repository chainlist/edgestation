<script lang="ts">
	import ChevronRight from '@lucide/svelte/icons/chevron-right';
	import Plus from '@lucide/svelte/icons/plus';
	import Trash from '@lucide/svelte/icons/trash';
	import PageHeader from '#lib/components/settings/PageHeader.svelte';
	import { Badge } from '#lib/components/ui/badge/index.ts';
	import { Button } from '#lib/components/ui/button/index.ts';
	import * as Card from '#lib/components/ui/card/index.ts';
	import { Input } from '#lib/components/ui/input/index.ts';
	import { profileUsers } from '#lib/layout/pages.ts';
	import type { Placement } from '#lib/layout/types.ts';
	import { addProfile, deleteProfile, renameProfile } from '#lib/state/profile-actions.ts';
	import { pages, profiles } from '#lib/state/settings.svelte.ts';
	import { installed, widgetName } from '#lib/widgets/installed.svelte.ts';
	import { usesWebview } from '#lib/widgets/permissions.ts';
	import { resolveSettings } from '#lib/widgets/settings.ts';

	/** The profile whose deletion waits for confirmation. */
	let confirming = $state<string>();
	let deleting = $state(false);
	let error = $state<string>();

	const webviews = $derived(new Set(installed.current.filter(usesWebview).map((w) => w.id)));

	/** The web view widgets in profile `id`, with their pages. `undefined` is the Default profile. */
	const usedBy = (id: string | undefined) => profileUsers(pages.current, id, webviews);

	/**
	 * What the placement shows, if its widget has a text setting, such as the address of the URL
	 * widget. Returns the value without `https://`, or an empty string if their is no text setting.
	 */
	function detail(item: Placement) {
		const schema = installed.current.find((w) => w.id === item.widget)?.settings ?? [];
		const text = schema.find((setting) => setting.type === 'text');
		if (!text) return '';
		const value = resolveSettings(schema, item.settings)[text.key] ?? '';
		return value.replace(/^https?:\/\//, '').replace(/\/$/, '');
	}

	function ask(id: string) {
		confirming = id;
		error = undefined;
	}

	async function remove(id: string) {
		deleting = true;
		error = undefined;
		try {
			await deleteProfile(id);
			confirming = undefined;
		} catch (e) {
			error = String(e);
		} finally {
			deleting = false;
		}
	}
</script>

<!-- The widgets that use a profile, one row for each, with the widget name, what it shows and its
     page. A row opens the widget settings in its page, where the user selects its profile. -->
{#snippet users(list: ReturnType<typeof usedBy>)}
	<Card.Content>
		{#if list.length}
			<ul
				class="grid grid-cols-[minmax(6rem,auto)_minmax(0,1fr)_auto_auto] gap-x-3 overflow-hidden rounded-lg border"
			>
				{#each list as { page, item } (item.id)}
					{@const shows = detail(item)}
					<li class="col-span-4 grid grid-cols-subgrid not-last:border-b">
						<a
							href="/settings/pages/{page.id}?widget={item.id}"
							class="col-span-4 grid grid-cols-subgrid items-center px-3 py-2 transition-colors hover:bg-muted/50"
						>
							<span class="font-medium">{widgetName(item.widget)}</span>
							<span class="truncate text-muted-foreground" title={shows}>{shows}</span>
							<span class="text-muted-foreground">{page.name}</span>
							<ChevronRight class="size-4 text-muted-foreground" />
						</a>
					</li>
				{/each}
			</ul>
		{:else}
			<p class="text-muted-foreground">No widget uses it.</p>
		{/if}
	</Card.Content>
{/snippet}

<PageHeader
	title="Browser profiles"
	description="Each profile keeps its own sign-ins and cookies, so two widgets showing the same site can use different accounts. Pick a widget's profile in its settings, under Pages."
>
	{#snippet actions()}
		<Button onclick={addProfile}><Plus />Add profile</Button>
	{/snippet}
</PageHeader>

<div class="grid gap-4">
	<Card.Root>
		<Card.Header>
			<Card.Title>Default</Card.Title>
			<Card.Description
				>The app's own browser data, used until you pick another profile.</Card.Description
			>
			<Card.Action><Badge variant="secondary">Built-in</Badge></Card.Action>
		</Card.Header>
		{@render users(usedBy(undefined))}
	</Card.Root>

	{#each profiles.current as profile (profile.id)}
		{@const list = usedBy(profile.id)}
		<Card.Root>
			<Card.Header>
				<!-- The name saves on Enter or on blur. -->
				<Input
					value={profile.name}
					class="w-64"
					aria-label="Name"
					onchange={(event) => {
						const name = event.currentTarget.value.trim();
						if (name) renameProfile(profile.id, name);
						else event.currentTarget.value = profile.name;
					}}
				/>
				<Card.Action>
					<Button
						size="icon-sm"
						variant="ghost"
						aria-label="Delete"
						disabled={deleting}
						onclick={() => ask(profile.id)}
					>
						<Trash />
					</Button>
				</Card.Action>
			</Card.Header>
			{@render users(list)}
			{#if confirming === profile.id}
				<Card.Content class="grid gap-3">
					<p class="text-sm">
						Deleting {profile.name} signs it out of every site and erases its data.
						{#if list.length}Its widgets switch to Default.{/if}
					</p>
					{#if error}
						<p class="text-sm text-destructive">Couldn't delete it: {error}</p>
					{/if}
					<div class="flex gap-2">
						<Button variant="destructive" disabled={deleting} onclick={() => remove(profile.id)}>
							{deleting ? 'Deleting…' : 'Delete profile'}
						</Button>
						<Button variant="outline" disabled={deleting} onclick={() => (confirming = undefined)}>
							Cancel
						</Button>
					</div>
				</Card.Content>
			{/if}
		</Card.Root>
	{/each}

	<p class="text-sm text-muted-foreground">
		Each profile with a widget on screen runs its own browser in the background, which takes memory:
		keep to the profiles you need.
	</p>
</div>
