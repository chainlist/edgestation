<script lang="ts">
	import ArrowRight from '@lucide/svelte/icons/arrow-right';
	import Cpu from '@lucide/svelte/icons/cpu';
	import Laptop from '@lucide/svelte/icons/laptop';
	import Monitor from '@lucide/svelte/icons/monitor';
	import RefreshCw from '@lucide/svelte/icons/refresh-cw';
	import Terminal from '@lucide/svelte/icons/terminal';
	import { onMount } from 'svelte';
	import { resolve } from '$app/paths';
	import BetaBadge from '#lib/components/BetaBadge.svelte';
	import DownloadButton from '#lib/components/DownloadButton.svelte';
	import GithubIcon from '#lib/components/GithubIcon.svelte';
	import InstallerLink from '#lib/components/InstallerLink.svelte';
	import { detectPlatform, linux, windows, type Platform } from '#lib/downloads.ts';
	import { RELEASE_URL, REPO_URL, VERSION } from '#lib/site.ts';
	import icon from '#lib/assets/app-icon.svg';

	// The page is prerendered, so it does not know the visitor at first. Windows is the default,
	// because most visitors use it. The browser then makes the button of its own platform primary.
	let platform = $state<Platform | undefined>('windows');
	onMount(() => {
		platform = detectPlatform(navigator.userAgent);
	});

	const linuxPage = resolve('/download/linux');

	const requirements = [
		{
			icon: Monitor,
			title: 'A secondary display',
			text: 'Made for the Corsair Xeneon Edge (2560 × 720, touch). Any other monitor works too, whatever its size or orientation.'
		},
		{
			icon: Laptop,
			title: 'Windows or Linux',
			text: 'A 64-bit PC with Windows, or with a Linux desktop on X11 or XWayland.'
		},
		{
			icon: Cpu,
			title: 'An NVIDIA GPU, for GPU stats',
			text: 'GPU load and temperature read the first NVIDIA GPU, through its driver. Everything else works without one.'
		}
	];

	const steps = [
		{
			title: 'Install and start EdgeStation',
			text: 'It opens full screen on your first secondary display, with a clock on its Home page.'
		},
		{
			title: 'Open Settings from the tray',
			text: 'Click the EdgeStation icon in the notification area (on Linux, choose Settings in its menu). Display picks another monitor if needed.'
		},
		{
			title: 'Build your pages',
			text: 'In Pages, choose a widget and add it, then drag and resize it on the grid. The Edge shows each change right away.'
		}
	];
</script>

<svelte:head>
	<title>Download EdgeStation</title>
	<meta
		name="description"
		content="Download EdgeStation for Windows and Linux: a widget dashboard for your secondary display."
	/>
</svelte:head>

<section class="relative overflow-hidden">
	<div
		class="pointer-events-none absolute inset-x-0 -top-40 h-[560px] bg-[radial-gradient(ellipse_at_center,oklch(0.55_0.2_260/0.3),transparent_65%)]"
		aria-hidden="true"
	></div>
	<div class="relative mx-auto max-w-3xl px-4 pt-20 pb-16 text-center sm:px-6 sm:pt-28">
		<img src={icon} alt="" class="mx-auto size-24 drop-shadow-[0_12px_40px_rgb(61_122_247/0.45)]" />
		<h1 class="mt-8 text-4xl font-bold tracking-tight sm:text-5xl">Download EdgeStation</h1>
		<p class="mt-4 text-lg text-muted-foreground">A widget dashboard for your secondary display.</p>
		<div class="mt-10 flex flex-col items-center justify-center gap-4 sm:flex-row sm:items-start">
			<div class="flex flex-col items-center gap-2">
				<DownloadButton
					href={windows.setup.url}
					size="lg"
					variant={platform === 'linux' ? 'secondary' : 'primary'}
				>
					Download for Windows
				</DownloadButton>
				<p class="text-sm text-muted-foreground">Setup program, 64-bit</p>
			</div>
			<div class="flex flex-col items-center gap-2">
				<DownloadButton
					href={linuxPage}
					size="lg"
					variant={platform === 'linux' ? 'primary' : 'secondary'}
				>
					Download for Linux
				</DownloadButton>
				<p class="text-sm text-muted-foreground">.deb, .rpm or AppImage</p>
			</div>
		</div>
		<p class="mt-6 text-sm text-muted-foreground">
			Version {VERSION}
			<BetaBadge /> ·
			<a href={RELEASE_URL} class="text-primary hover:underline">Release on GitHub</a>
		</p>
	</div>
</section>

<div class="mx-auto max-w-5xl space-y-20 px-4 pb-28 sm:px-6">
	<section>
		<h2 class="text-2xl font-bold tracking-tight">Pick your installer</h2>
		<p class="mt-2 text-muted-foreground">
			Each way to install has its own file. A click on a file downloads it.
		</p>
		<div class="mt-6 grid gap-4 md:grid-cols-2">
			<div class="flex flex-col rounded-2xl border bg-card p-5">
				<h3 class="font-semibold">Windows</h3>
				<div class="mt-3 mb-5 space-y-1">
					<InstallerLink installer={windows.setup} text="The setup program." />
					<InstallerLink installer={windows.msi} text="A Windows Installer package." />
				</div>
				<p class="mt-auto border-t pt-4 text-sm text-muted-foreground">
					EdgeStation draws its windows with Microsoft Edge WebView2, which comes with Windows 11.
				</p>
			</div>
			<div class="flex flex-col rounded-2xl border bg-card p-5">
				<h3 class="font-semibold">Linux</h3>
				<div class="mt-3 mb-5 space-y-1">
					<InstallerLink installer={linux.deb} text="For Debian, Ubuntu and other apt systems." />
					<InstallerLink installer={linux.rpm} text="For Fedora, openSUSE and other rpm systems." />
					<InstallerLink
						installer={linux.appImage}
						text="For any distribution, without installing."
					/>
				</div>
				<a
					href={linuxPage}
					class="mt-auto inline-flex items-center gap-1.5 border-t pt-4 text-sm font-medium text-primary hover:underline"
				>
					How to install each one, and what EdgeStation needs on Linux
					<ArrowRight class="size-4 shrink-0" />
				</a>
			</div>
		</div>
		<div class="mt-4 flex gap-3 rounded-2xl border bg-card/40 p-5">
			<RefreshCw class="mt-0.5 size-5 shrink-0 text-primary" />
			<p class="text-sm text-muted-foreground">
				EdgeStation updates itself each time it opens, from the same kind of file you installed: the
				setup program from the setup program, the .msi from the .msi, the .deb from the .deb, the
				.rpm from the .rpm and the AppImage from the AppImage. The .deb and the .rpm ask for your
				password to install an update.
			</p>
		</div>
	</section>

	<section>
		<h2 class="text-2xl font-bold tracking-tight">What you need</h2>
		<div class="mt-6 grid gap-4 md:grid-cols-3">
			{#each requirements as { icon: Icon, title, text } (title)}
				<div class="rounded-2xl border bg-card p-5">
					<Icon class="size-5 text-primary" />
					<h3 class="mt-3 font-semibold">{title}</h3>
					<p class="mt-1.5 text-sm text-muted-foreground">{text}</p>
				</div>
			{/each}
		</div>
	</section>

	<section id="get-started">
		<h2 class="text-2xl font-bold tracking-tight">Get started</h2>
		<ol class="mt-6 grid gap-4 md:grid-cols-3">
			{#each steps as { title, text }, i (title)}
				<li class="rounded-2xl border bg-card p-5">
					<span
						class="grid size-8 place-items-center rounded-full bg-primary text-sm font-bold text-primary-foreground"
					>
						{i + 1}
					</span>
					<h3 class="mt-4 font-semibold">{title}</h3>
					<p class="mt-1.5 text-sm text-muted-foreground">{text}</p>
				</li>
			{/each}
		</ol>
	</section>

	<section class="grid items-center gap-8 rounded-2xl border bg-card/40 p-6 sm:p-8 md:grid-cols-2">
		<div>
			<div class="flex items-center gap-2 text-primary">
				<Terminal class="size-5" />
				<h2 class="text-xl font-bold tracking-tight text-foreground">Build it from source</h2>
			</div>
			<p class="mt-3 text-sm text-muted-foreground">
				With Node.js, pnpm and Rust installed, clone the repository and build the installer
				yourself. On Linux, first install the packages that the
				<a href="{REPO_URL}#develop" class="text-primary hover:underline">README</a> lists.
			</p>
			<a
				href={REPO_URL}
				class="mt-5 inline-flex h-10 items-center gap-2 rounded-lg border bg-card px-4 text-sm font-semibold transition hover:bg-muted"
			>
				<GithubIcon class="size-4" />chainlist/edgestation
			</a>
		</div>
		<pre
			class="overflow-x-auto rounded-xl border bg-background p-4 font-mono text-sm leading-relaxed"><code
				><span class="text-muted-foreground">$</span> pnpm install
<span class="text-muted-foreground">$</span> pnpm tauri build</code
			></pre>
	</section>
</div>
