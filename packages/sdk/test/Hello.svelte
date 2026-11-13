<script lang="ts">
	import type { WidgetProps } from '../src/index.js';

	let { api }: WidgetProps = $props();
</script>

<p>{api.instanceId}</p>
