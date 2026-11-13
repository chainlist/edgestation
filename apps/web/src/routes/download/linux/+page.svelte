<script lang="ts">
	import ArrowLeft from '@lucide/svelte/icons/arrow-left';
	import ArrowRight from '@lucide/svelte/icons/arrow-right';
	import RefreshCw from '@lucide/svelte/icons/refresh-cw';
	import { resolve } from '$app/paths';
	import Command from '#lib/components/Command.svelte';
	import DownloadButton from '#lib/components/DownloadButton.svelte';
	import { linux, type Installer } from '#lib/downloads.ts';
	import { RELEASE_URL, VERSION } from '#lib/site.ts';
	import icon from '#lib/assets/app-icon.svg';

	type Package = {
		installer: Installer;
		/** The text of the download button. */
		kind: string;
		title: string;
		text: string;
		/** How to install the file, from the folder it was downloaded to. */
		install: { label?: string; lines: string[] }[];
		note?: string;
	};

	const packages: Package[] = [
		{
			installer: linux.deb,
			kind: '.deb',
			title: 'Debian, Ubuntu, Linux Mint',
			text: 'For Debian, Ubuntu, Linux Mint, Pop!_OS and other apt systems. apt installs what it needs.',
			install: [{ lines: [`sudo apt install ./${linux.deb.file}`] }]
		},
		{
			installer: linux.rpm,
			kind: '.rpm',
			title: 'Fedora, openSUSE',
			text: 'For Fedora, openSUSE and other rpm systems. dnf or zypper installs what it needs.',
			install: [
				{ label: 'Fedora', lines: [`sudo dnf install ./${linux.rpm.file}`] },
				{ label: 'openSUSE', lines: [`sudo zypper install ./${linux.rpm.file}`] }
			]
		},
		{
			installer: linux.appImage,
			kind: 'AppImage',
			title: 'Any distribution',
			text: 'One file that runs without installing, on any distribution with glibc 2.35 or later.',
			install: [
				{
					lines: [`chmod +x ${linux.appImage.file}`, `./${linux.appImage.file}`]
				}
			],
			note: 'The AppImage needs libfuse2, which recent distributions no longer install by default (libfuse2t64 on Ubuntu 24.04).'
		}
	];

	/** What the Linux side reads the machine with, from ARCHITECTURE.md (Platforms). */
	const needs = [
		{
			title: 'Display',
			text: 'EdgeStation runs on X11 desktops, and Wayland sessions run it on XWayland. It draws its windows with WebKitGTK 4.1.'
		},
		{ title: 'Sound', text: 'The Mixer works with PulseAudio, or PipeWire with pipewire-pulse.' },
		{ title: 'Media', text: 'The Media widget reads players through MPRIS.' },
		{ title: 'Wi-Fi', text: 'The network name comes from NetworkManager.' },
		{
			title: 'Tray',
			text: 'The tray icon needs an app indicator host. On a desktop without one, start EdgeStation again to open Settings.'
		},
		{
			title: 'GPU stats',
			text: 'GPU load and temperature read the first NVIDIA GPU, through its driver. Everything else works without one.'
		}
	];
</script>

<svelte:head>
	<title>Download EdgeStation for Linux</title>
	<meta
		name="description"
		content="Download EdgeStation for Linux as a .deb, an .rpm or an AppImage, and install it."
	/>
</svelte:head>

<section class="relative overflow-hidden">
	<div
		class="pointer-events-none absolute inset-x-0 -top-40 h-[560px] bg-[radial-gradient(ellipse_at_center,oklch(0.55_0.2_260/0.3),transparent_65%)]"
		aria-hidden="true"
	></div>
	<div class="relative mx-auto max-w-3xl px-4 pt-14 pb-14 text-center sm:px-6 sm:pt-20">
		<a
			href={resolve('/download')}
			class="inline-flex items-center gap-1.5 text-sm text-muted-foreground transition hover:text-foreground"
		>
			<ArrowLeft class="size-4" />All downloads
		</a>
		<img
			src={icon}
			alt=""
			class="mx-auto mt-8 size-20 drop-shadow-[0_12px_40px_rgb(61_122_247/0.45)]"
		/>
		<h1 class="mt-8 text-4xl font-bold tracking-tight sm:text-5xl">EdgeStation for Linux</h1>
		<p class="mt-4 text-lg text-pretty text-muted-foreground">
			Pick the package of your distribution. Each one is for 64-bit PCs (x86_64).
		</p>
		<p class="mt-3 text-sm text-muted-foreground">
			Version {VERSION} ·
			<a href={RELEASE_URL} class="text-primary hover:underline">Release on GitHub</a>
		</p>
	</div>
</section>

<div class="mx-auto max-w-6xl space-y-20 px-4 pb-28 sm:px-6">
	<section class="grid gap-4 lg:grid-cols-3">
		{#each packages as { installer, kind, title, text, install, note } (kind)}
			<div class="flex min-w-0 flex-col rounded-2xl border bg-card p-5 sm:p-6">
				<p class="font-mono text-sm text-primary">{kind}</p>
				<h2 class="mt-1 text-xl font-bold tracking-tight">{title}</h2>
				<p class="mt-2 text-sm text-muted-foreground">{text}</p>
				<div class="mt-5">
					<DownloadButton href={installer.url}>Download {kind}</DownloadButton>
				</div>
				<p class="mt-2 font-mono text-xs break-all text-muted-foreground">{installer.file}</p>
				<div class="mt-6 border-t pt-5">
					<h3 class="text-sm font-semibold">Install it</h3>
					<p class="mt-1 text-sm text-muted-foreground">
						In a terminal, in the folder of the file:
					</p>
					<div class="mt-3 space-y-3">
						{#each install as { label, lines } (lines[0])}
							<div>
								{#if label}
									<p class="mb-1.5 text-xs font-medium text-muted-foreground">{label}</p>
								{/if}
								<Command {lines} />
							</div>
						{/each}
					</div>
					{#if note}
						<p class="mt-3 text-sm text-muted-foreground">{note}</p>
					{/if}
				</div>
			</div>
		{/each}
	</section>

	<section>
		<div class="flex gap-3 rounded-2xl border bg-card/40 p-5">
			<RefreshCw class="mt-0.5 size-5 shrink-0 text-primary" />
			<p class="text-sm text-muted-foreground">
				EdgeStation updates itself each time it opens, from the same kind of file you installed: the
				.deb from the .deb, the .rpm from the .rpm and the AppImage from the AppImage. The .deb and
				the .rpm ask for your password to install an update.
			</p>
		</div>
	</section>

	<section>
		<h2 class="text-2xl font-bold tracking-tight">What EdgeStation uses on Linux</h2>
		<dl class="mt-6 grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
			{#each needs as { title, text } (title)}
				<div class="rounded-2xl border bg-card p-5">
					<dt class="font-semibold">{title}</dt>
					<dd class="mt-1.5 text-sm text-muted-foreground">{text}</dd>
				</div>
			{/each}
		</dl>
	</section>

	<section
		class="flex flex-col items-start gap-4 rounded-2xl border bg-card/40 p-6 sm:flex-row sm:items-center sm:p-8"
	>
		<div>
			<h2 class="text-xl font-bold tracking-tight">Then build your pages</h2>
			<p class="mt-2 text-sm text-muted-foreground">
				Start EdgeStation from your app menu, or the AppImage from its file. It opens full screen on
				your first secondary display. Choose Settings in the menu of its tray icon to add widgets.
			</p>
		</div>
		<a
			href="{resolve('/download')}#get-started"
			class="inline-flex h-10 shrink-0 items-center gap-2 rounded-lg border bg-card px-4 text-sm font-semibold transition hover:bg-muted sm:ml-auto"
		>
			Get started <ArrowRight class="size-4" />
		</a>
	</section>
</div>
