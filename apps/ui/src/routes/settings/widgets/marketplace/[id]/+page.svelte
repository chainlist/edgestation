<script lang="ts">
	import WidgetDetail from '#lib/components/settings/widgets/WidgetDetail.svelte';
	import type { PageProps } from './$types';

	let { params }: PageProps = $props();
</script>

<WidgetDetail id={params.id} back="/settings/widgets/marketplace" />
