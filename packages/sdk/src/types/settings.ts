/** An option with a value for each placement. The manifest declares it. */
export type WidgetSetting = SelectSetting | TextSetting;

interface SettingBase {
	key: string;
	label: string;
	/**
	 * The setting shows only while this condition is true. At other times, its value has no
	 * meaning.
	 */
	when?: SettingCondition;
	/**
	 * The setting does not show while this condition is true. At these times, its value has no
	 * meaning. Use it when the values that hide the setting are fewer than the values that show
	 * it, or when the other setting has a `source`.
	 */
	unless?: SettingCondition;
}

/**
 * True while each named setting has the given value, or one of the values in a list. For
 * example, `{ display: ['faders', 'buttons'] }`.
 */
export type SettingCondition = Record<string, string | string[]>;

/** A setting with one value from a list. */
export interface SelectSetting extends SettingBase {
	type: 'select';
	options: { value: string; label: string }[];
	/**
	 * Adds the choices of a list after `options`. EdgeStation reads this list when the settings
	 * show. A saved value can be missing from the list, for example an app that does not run.
	 * This value stays saved, and the widget gets it as it is.
	 */
	source?: SettingSource;
	/** The value until the user selects an option. If you do not set it, it is the first option. */
	default?: string;
}

/**
 * The lists that a select setting can add choices from:
 * - `audioApps`: the apps with sound on the default output (`api.audio`). The setting saves
 *   each app as the file name of its executable (`AudioApp.program`, such as `Discord.exe`).
 */
export type SettingSource = 'audioApps';

/** Free text, such as a URL. */
export interface TextSetting extends SettingBase {
	type: 'text';
	/** The value until the user types something. If you do not set it, the value is empty. */
	default?: string;
	/** A hint that shows while the field is empty. */
	placeholder?: string;
	/**
	 * Hides the value in the settings, as in a password field. Use it for a token or a key.
	 * EdgeStation saves the value as plain text in its settings file.
	 */
	secret?: boolean;
}

/** The saved setting values of one placement, by key. */
export type WidgetSettingValues = Record<string, string>;
