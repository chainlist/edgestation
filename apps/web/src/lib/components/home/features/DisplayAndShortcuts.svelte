<script lang="ts">
	import Keyboard from '@lucide/svelte/icons/keyboard';
	import Snowflake from '@lucide/svelte/icons/snowflake';
	import Screenshot from '#lib/components/Screenshot.svelte';
	import settingsGeneral from '#lib/assets/settings-general.png';
	import settingsShortcuts from '#lib/assets/settings-shortcuts.png';
</script>

<section>
	<div class="max-w-2xl">
		<p class="text-sm font-semibold text-primary">Any display</p>
		<h2 class="mt-2 text-3xl font-bold tracking-tight sm:text-4xl">
			Made for the Edge, at home anywhere
		</h2>
		<p class="mt-4 text-muted-foreground">
			EdgeStation takes the first secondary display, or the one you pick. Any size, shape or
			orientation works, with or without touch, and widgets lay themselves out in their cell.
		</p>
	</div>
	<div class="mt-10 grid gap-8 lg:grid-cols-2">
		<div>
			<Screenshot
				src={settingsGeneral}
				alt="General settings: open at startup, pause widgets while another app is full screen, and updates."
			/>
			<div class="mt-6 flex gap-4">
				<Snowflake class="mt-0.5 size-5 shrink-0 text-primary" />
				<p class="text-sm text-muted-foreground">
					<strong class="text-foreground">Out of the way while you play.</strong> When a game or video
					goes full screen, widgets pause and the Edge shows a Paused badge. Switch apps and they carry
					on.
				</p>
			</div>
		</div>
		<div>
			<Screenshot
				src={settingsShortcuts}
				alt="Shortcut settings: previous page, next page, go to page, bring to front, and hide or show, each with its keys."
			/>
			<div class="mt-6 flex gap-4">
				<Keyboard class="mt-0.5 size-5 shrink-0 text-primary" />
				<p class="text-sm text-muted-foreground">
					<strong class="text-foreground">Shortcuts from any app.</strong> Turn pages with
					<kbd>Ctrl</kbd> <kbd>Alt</kbd> <kbd>Page Up</kbd> / <kbd>Page Down</kbd>, type a page's
					name after <kbd>Ctrl</kbd> <kbd>Alt</kbd> <kbd>G</kbd> to go straight to it, and bring the
					Edge back to the front with <kbd>Ctrl</kbd> <kbd>Alt</kbd> <kbd>Home</kbd>, or record your
					own.
				</p>
			</div>
		</div>
	</div>
</section>

<style>
	kbd {
		border: 1px solid var(--border);
		border-radius: 0.375rem;
		background: var(--muted);
		padding: 0.05rem 0.35rem;
		font-family: inherit;
		font-size: 0.75rem;
		color: var(--foreground);
	}
</style>
