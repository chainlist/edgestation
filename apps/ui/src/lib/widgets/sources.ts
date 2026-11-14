import { mixer } from './api/audio';
import type { SelectSetting, SettingSource } from './types';

type Choice = SelectSetting['options'][number];

/** A list that a select setting can add choices from (its `source`). */
interface Source {
	read(): Promise<Choice[]>;
	/** The label for a saved value that is not in the choices now. */
	missing(value: string): string;
}

const SOURCES: Record<SettingSource, Source> = {
	audioApps: {
		async read() {
			const { apps } = await mixer();
			const seen = new Set<string>();
			const choices: Choice[] = [];
			// The list is already in name order. All apps of one program give one choice, for
			// example two apps that embed the same browser.
			for (const { program, name } of apps) {
				if (!program || seen.has(program.toLowerCase())) continue;
				seen.add(program.toLowerCase());
				choices.push({ value: program, label: name });
			}
			return choices;
		},
		missing: (value) => `${value.replace(/\.exe$/i, '')} (not running)`
	}
};

/**
 * Returns the choices that `source` adds now. Returns none if the read fails, or if this app
 * version does not know the source, for example in a widget for a newer app.
 */
export async function readSource(source: SettingSource): Promise<Choice[]> {
	try {
		return (await SOURCES[source]?.read()) ?? [];
	} catch {
		return [];
	}
}

/**
 * Returns the choices that a select shows. First come its own options, then the choices from
 * its source (`read`, `undefined` until the read is done). Last comes the saved value if it is
 * in niether list. After the read, it shows as missing.
 */
export function sourcedChoices(
	setting: SelectSetting,
	read: Choice[] | undefined,
	saved: string | undefined
): Choice[] {
	const own = new Set(setting.options.map((option) => option.value));
	const choices = [...setting.options, ...(read ?? []).filter((choice) => !own.has(choice.value))];
	if (saved && !choices.some((choice) => choice.value === saved)) {
		const source = setting.source && SOURCES[setting.source];
		choices.push({ value: saved, label: read && source ? source.missing(saved) : saved });
	}
	return choices;
}
