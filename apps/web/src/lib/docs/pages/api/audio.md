`api.audio` is the machine's sound as the system's volume mixer shows it: the default output and microphone, the apps playing through that output, their volumes and how loud they are right now. On Windows, it comes from Windows' volume mixer. On Linux, it comes from PulseAudio, which PipeWire also serves through pipewire-pulse: the same devices and app streams as the sound settings of GNOME and KDE, or pavucontrol. Read it once, then again when it changes: the system tells the app, so nothing polls. It needs `"permissions": ["audio"]` in the [manifest](/docs/manifest#permissions).

```js
const show = api.guard(async () => {
	const { output, apps } = await api.audio.mixer();
	el.textContent = output
		? `${Math.round(output.volume * 100)} % · ${apps.length} apps`
		: 'No output';
});
show();
api.audio.onChange(show);
```

## Members

| Member                      | What it does                                                                                                                                         |
| --------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------- |
| `mixer()`                   | Promise of the default output and microphone, and the apps on that output (see below).                                                               |
| `onChange(fn)`              | Calls `fn` when the mixer changes, to read `mixer()` again (see below). Returns a function that stops it.                                            |
| `onLevels(fn)`              | Calls `fn` with the levels 25 times a second while there is sound, for meters (see below). Returns a function that stops it.                         |
| `levels()`                  | Promise of how loud the output, the microphone and each app are right now.                                                                           |
| `setVolume(target, volume)` | Sets the volume of `'output'`, `'input'` or an app's `id`, from 0 to 1. Resolves `false` when it is gone (the app closed, the device was unplugged). |
| `setMuted(target, muted)`   | Mutes or unmutes the same targets. Resolves `false` when it is gone.                                                                                 |
| `outputs()`                 | Promise of the output devices plugged in and enabled, by name: `{ id, name }`.                                                                       |
| `setOutput(id)`             | Makes output `id` the default one, as the sound settings do ([see below](#switching-output)). Resolves `false` when it is no longer plugged in.      |
| `icon(path, size?)`         | URL of the icon of an app's executable (its `path`), a PNG `size` pixels square (16 to 256, default 64), for an `<img>`.                             |

## The mixer

`mixer()` resolves to `{ output, input, apps }`. `output` and `input` are the default devices, or `null` when there is none:

| Field    | Meaning                                                                                 |
| -------- | --------------------------------------------------------------------------------------- |
| `id`     | The device's id, as `outputs()` lists it: Windows' id, or the PulseAudio name on Linux. |
| `name`   | What the sound settings call it (`Speakers (Realtek Audio)` on Windows).                |
| `volume` | From 0 to 1, as the system's volume slider shows it.                                    |
| `muted`  | `true` while muted.                                                                     |

On Linux, `volume` is that of the loudest channel, and 1 when the sound settings raise it above 100 %. `setVolume` keeps the balance between channels.

`apps` has one entry per app with sound on the default output, playing or not, by name. On Windows, an app that opened several sessions counts once, as in Windows' volume mixer, and System sounds come last. On Linux, all the streams of one executable count as one app, and a stream that doesn't name its executable counts on its own:

| Field     | Meaning                                                                                                                                                                                                                     |
| --------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `id`      | Names the app's sound while it runs, for `setVolume`, `setMuted` and `levels()`. It is not kept across restarts: don't save it.                                                                                             |
| `name`    | The name the app gives its sound, else what Task Manager calls it (on Linux, its name in the app menu), else its executable's file name.                                                                                    |
| `path`    | Its executable, for `icon()`; `null` for System sounds, and on Linux for an app that runs as another user.                                                                                                                  |
| `program` | Its executable's file name (`Discord.exe` on Windows, `firefox` on Linux); `null` for System sounds, and on Linux for a stream that doesn't name it. It names the app across restarts and updates: save this one, not `id`. |
| `volume`  | From 0 to 1, a share of the output's volume.                                                                                                                                                                                |
| `muted`   | `true` while muted.                                                                                                                                                                                                         |
| `active`  | `true` while it plays, `false` while it keeps its sound open without playing.                                                                                                                                               |

On Linux, `icon()` draws the icon of the app's entry in the app menu, else the icon of the theme named like its executable.

## The microphone

`input` is the default microphone. On Windows, calls can use another one, the default device for communication, so muting `'input'` mutes both when they differ, and `input.muted` is `true` only when both are. On Linux, `input` is the default input, or `null` when that is the monitor of an output rather than a microphone. Muting `'input'` mutes the device itself, so it works in every app; an app's own mute button doesn't show it.

```js
micButton.addEventListener(
	'click',
	api.guard(async () => {
		const { input } = await api.audio.mixer();
		if (input) await api.audio.setMuted('input', !input.muted);
	})
);
```

## Changes

`onChange(fn)` calls `fn` when a volume or a mute changes, an app opens, closes, starts or stops its sound, or the default device changes, from this widget, another app or the system itself. Read `mixer()` then. The system calls the app back (on Linux, through a PulseAudio subscription), so nothing runs while nothing changes. A burst of changes (a volume slider dragged) calls it at most every 100 ms. On Linux, it also calls `fn` when the sound server comes back after a restart, since the mixer can differ then.

While the widget's page is off screen or the Edge is frozen, it doesn't listen; it calls `fn` once when the widget shows again, for what changed meanwhile. It stops when the widget unmounts.

Calls of `mixer()` less than 250 ms apart share one read and get the same result, whichever widgets make them: several mixers reading after the same change cost one read. A change made with `setVolume`, `setMuted` or `setOutput` is in the next `mixer()` read.

## Level meters

Neither Windows nor PulseAudio sends levels as they change, so `onLevels(fn)` has the app read them: 25 times a second while there is sound, and for two seconds after, so meters can fall. Then it reads four times a second and sends nothing until there is sound again. It only runs while a widget listens and its page shows. Each call gets `{ output, input, apps }`: the peak of the last few milliseconds, from 0 to 1, for the output, the microphone (0 unless an app records from it) and each app by `id`.

```js
api.audio.onLevels(({ output }) => {
	bar.style.transform = `scaleY(${output})`;
});
```

Listen only while something can sound: an app's `active` says it has a stream open. Peaks are linear, which makes quiet sound look like nothing: the decibels, `20 * Math.log10(peak)`, follow what ears hear. Move meters with transforms rather than repainting them: they change many times a second. `levels()` reads them once, for a value rather than a meter.

## Switching output

```js
const outputs = await api.audio.outputs();
const headset = outputs.find((output) => output.name.includes('Headset'));
if (headset) await api.audio.setOutput(headset.id);
```

`setOutput` changes the default device. On Windows, it works like Sound settings' "Set as default device": calls keep the device picked for them as the default for communication. Windows has no public API for it, so `setOutput` uses the one Sound settings itself uses, which other volume tools rely on too.

On Linux, `setOutput` sets PulseAudio's default output. The apps that follow the default move to it, and an app the user moved to a device stays there.

The [`mixer` widget](https://github.com/chainlist/edgestation/tree/main/widgets/mixer) is a complete example, with sliders, meters, mute buttons and an output picker.
