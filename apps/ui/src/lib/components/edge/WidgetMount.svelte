<script lang="ts">
	import type { LoadingLook } from '#lib/widgets/api/loading.ts';
	import type { Pause } from '#lib/widgets/api/timers.ts';
	import { mountWidget } from '#lib/widgets/mount.ts';
	import WidgetLoading from './WidgetLoading.svelte';

	let {
		widget,
		instance,
		settings,
		profile,
		pause
	}: {
		widget: string;
		instance: string;
		settings?: Record<string, string>;
		profile?: string;
		pause: Pause;
	} = $props();

	// The settings sync replaces all page objects. These derived values change only when the ids
	// or the settings text change. Thus an unrelated edit does not mount the widget again.
	const widgetId = $derived(widget);
	const instanceId = $derived(instance);
	const settingsJson = $derived(JSON.stringify(settings ?? {}));
	const profileId = $derived(profile);

	// Async errors cannot get to <svelte:boundary> on thier own. Keep them, then throw them again
	// in an effect, because the boundary catches errors from effects.
	let crash = $state<{ error: unknown }>();
	$effect(() => {
		if (crash) throw crash.error;
	});

	let loading = $state<LoadingLook>();
	const hidden = $derived(loading === 'dots');
</script>

<div class="relative size-full">
	<!-- The widget stays hidden under the dots until its first load ends. Only the fade in has a
	     transition. Else the content that the widget draws on mount fades out. -->
	<div
		class={['size-full', hidden ? 'opacity-0' : 'transition-opacity duration-300']}
		inert={hidden}
		{@attach (host) =>
			mountWidget(
				host,
				widgetId,
				instanceId,
				JSON.parse(settingsJson),
				profileId,
				pause,
				(error) => (crash = { error }),
				(look) => (loading = look)
			)}
	></div>
	<WidgetLoading look={loading} />
</div>
