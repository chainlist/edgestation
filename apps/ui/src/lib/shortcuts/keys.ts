/**
 * A system-wide shortcut in the format of the global shortcut plugin: the modifiers, then the
 * `KeyboardEvent.code` of the key, for example "Control+Alt+PageDown".
 */
export type Shortcut = string;

/** The parts of a key press that `record` reads. */
export type KeyPress = Pick<
	KeyboardEvent,
	'code' | 'keyCode' | 'ctrlKey' | 'altKey' | 'shiftKey' | 'metaKey'
>;

/** The state of the shortcut recording after a key press. */
export type Recording =
	/** Only modifiers so far, with the labels of the held keys. */
	{ held: string[] } | { shortcut: Shortcut } | { error: string };

const MODIFIERS = [
	{ name: 'Control', label: 'Ctrl', held: (e: KeyPress) => e.ctrlKey },
	{ name: 'Alt', label: 'Alt', held: (e: KeyPress) => e.altKey },
	{ name: 'Shift', label: 'Shift', held: (e: KeyPress) => e.shiftKey },
	{ name: 'Super', label: 'Win', held: (e: KeyPress) => e.metaKey }
];

const MODIFIER_KEY = /^(Control|Alt|Shift|Meta|OS)(Left|Right)?$/;

/**
 * The keys other than letters that the plugin finds in the same place on all layouts. There is
 * no punctuation, because the plugin looks for it where a US keyboard has it. On other layouts,
 * that key types somthing different.
 */
const KEYS =
	/^(Digit\d|F([1-9]|1\d|2[0-4])|Numpad(\d|Add|Decimal|Divide|Enter|Multiply|Subtract)|Arrow(Up|Down|Left|Right)|Page(Up|Down)|Home|End|Insert|Delete|Backspace|Enter|Space|Pause|PrintScreen|ScrollLock|AudioVolume(Up|Down|Mute)|Media(PlayPause|Stop|TrackNext|TrackPrevious))$/;

/** Keys that no app uses, so they can be a shortcut alone. */
const ALONE = /^F(1[3-9]|2[0-4])$/;

/** Uses the letter a key types, not its position, as the plugin does. A is A on AZERTY too. */
function key(e: KeyPress): string | null {
	if (e.keyCode >= 65 && e.keyCode <= 90) return `Key${String.fromCharCode(e.keyCode)}`;
	return KEYS.test(e.code) ? e.code : null;
}

/** Turns a key press into a shorcut, or gives the reason why it cannot be one. */
export function record(e: KeyPress): Recording {
	const mods = MODIFIERS.filter((m) => m.held(e));
	if (MODIFIER_KEY.test(e.code)) return { held: mods.map((m) => m.label) };
	const main = key(e);
	if (!main) return { error: "This key can't be part of a shortcut." };
	if (!mods.some((m) => m.name !== 'Shift') && !ALONE.test(main)) {
		return { error: 'Hold Ctrl, Alt or Win with it, so the key still works in other apps.' };
	}
	return { shortcut: [...mods.map((m) => m.name), main].join('+') };
}

/** Gives the keys of a shortcut as a keyboard shows them: ["Ctrl", "Alt", "Page Down"]. */
export function keyLabels(shortcut: Shortcut): string[] {
	return shortcut
		.split('+')
		.map(
			(part) =>
				MODIFIERS.find((m) => m.name === part)?.label ??
				part.replace(/^(Key|Digit|Arrow|Audio|Media)/, '').replace(/([a-z])([A-Z\d])/g, '$1 $2')
		);
}
