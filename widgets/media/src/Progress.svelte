<script lang="ts">
	import { clock } from './clock.ts';

	let { position, duration }: { position: number; duration: number } = $props();
</script>

<div class="progress">
	<div class="bar">
		<div class="fill" style:transform="scaleX({position / duration})"></div>
	</div>
	<p class="times">
		<span>{clock(position)}</span><span>{clock(duration)}</span>
	</p>
</div>

<style>
	.progress {
		margin-top: min(2cqh, 1.5cqw);
	}

	.bar {
		height: max(3px, 1.5cqh);
		overflow: hidden;
		border-radius: 999px;
		background: var(--muted);
	}

	.fill {
		height: 100%;
		background: var(--primary);
		transform-origin: left;
	}

	.times {
		display: flex;
		justify-content: space-between;
		margin-top: 0.4em;
		color: var(--muted-foreground);
		font-size: min(6.5cqh, 3.5cqw);
		font-variant-numeric: tabular-nums;
	}

	@container (aspect-ratio < 1.4) {
		.times {
			font-size: min(4.5cqh, 5.5cqw);
		}
	}
</style>
