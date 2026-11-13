/** Formats seconds as m:ss, or as h:mm:ss for one hour or more. */
export function clock(seconds: number) {
	const s = Math.floor(seconds);
	const pad = (n: number) => String(n).padStart(2, '0');
	const [h, m] = [Math.floor(s / 3600), Math.floor(s / 60) % 60];
	return h ? `${h}:${pad(m)}:${pad(s % 60)}` : `${m}:${pad(s % 60)}`;
}
