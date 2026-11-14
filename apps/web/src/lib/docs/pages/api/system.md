Two calls read how hard the machine is working: `api.system()` for CPU, GPU, network and Wi-Fi, and `api.topPrograms()` for the programs behind the CPU load. Both read the machine when called; nothing monitors it in between. Both need `"permissions": ["system"]` in the [manifest](/docs/manifest#permissions).

## `api.system()`

```ts
system(): Promise<SystemInfo>
```

```js
api.every(1000, async () => {
	const { cpu } = await api.system();
	el.textContent = `${Math.round(cpu.load)} %`;
});
```

CPU load and network speeds are differences between two reads, so they cover the time since the previous call, from any widget. Calls less than 500 ms apart share one read, so several widgets polling every second cost one read per second. Keep a history yourself if you want to chart a value.

| Field                            | Meaning                                                                                                                                                  |
| -------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `hostname`                       | The machine's name.                                                                                                                                      |
| `wifi`                           | The connected Wi-Fi network, or `null`: the name of its Windows profile, or of its NetworkManager connection on Linux. Both are the SSID unless renamed. |
| `cpu.load`                       | Load of all cores, in percent.                                                                                                                           |
| `gpu.load`, `gpu.temperature`    | The first NVIDIA GPU: percent and °C. `gpu` is `null` without one or without its driver.                                                                 |
| `net.down`, `net.up`             | Internet traffic since the previous call, in bytes per second.                                                                                           |
| `net.avg1m`, `.avg5m`, `.avg15m` | `{ down, up }` averaged over the calls of the last 1, 5 or 15 minutes.                                                                                   |

The averages only know the calls widgets made: they cover the full window while a widget keeps polling, and less right after polling starts or resumes, for example when a page comes back on screen.

On Linux, `wifi` is also `null` without NetworkManager, and `net` counts only the interfaces of a device: not the loopback, nor the virtual ones of Docker or virtual machines, whose traffic also goes through a real interface or never leaves the machine.

CPU temperature is not available, on either system: Windows doesn't expose it to normal apps.

### A network meter

```js
const format = (bytes) =>
	bytes > 1e6 ? `${(bytes / 1e6).toFixed(1)} MB/s` : `${Math.round(bytes / 1e3)} kB/s`;

api.every(1000, async () => {
	const { net } = await api.system();
	el.textContent = `↓ ${format(net.down)}  ↑ ${format(net.up)}  (1 min ↓ ${format(net.avg1m.down)})`;
});
```

## `api.topPrograms()`

```ts
topPrograms(count: number): Promise<ProgramUsage[]>
```

The programs that used the most CPU since the previous call (from any widget), busiest first: at most `count`, fewer when not that many used any.

```js
api.every(1000, async () => {
	const [top] = await api.topPrograms(1);
	el.textContent = top ? `${top.name} · ${Math.round(top.cpu)} %` : 'Idle';
});
```

| Field  | Meaning                                                                                                                                                                                                                                                                                  |
| ------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name` | What Task Manager calls it: the executable's description (`THE FINALS`, `Google Chrome`), else its file name without `.exe` (`crashpad_handler`). On Linux, the name of its app in the app menu, else the executable's file name. All the processes with that name count as one program. |
| `cpu`  | Percent of all logical processors, like Task Manager.                                                                                                                                                                                                                                    |

Services and system processes are included, as in Task Manager. On Linux, the processes of every user are included too.

Like `api.system()`, it reads when called, and calls less than 500 ms apart share one read. It lists every process on the machine, which costs more than `api.system()`, so it is a separate call: only make it while you show the result.
