/** Where a widget saves data. Select the scope that fits each value. */
export interface StorageApi {
	/** Shared by all placements of this widget. */
	readonly widget: WidgetStorage;
	/** Only for this placement. Other copies of the widget do not see it. */
	readonly instance: WidgetStorage;
}

/** Saved data: JSON values by key. The host writes them to disk shortly after each change. */
export interface WidgetStorage {
	/** The value saved under `key`. `undefined` if there is none. */
	get<T = unknown>(key: string): Promise<T | undefined>;
	/** Saves `value` under `key` and replaces the previous value. JSON must be able to hold it. */
	set(key: string, value: unknown): Promise<void>;
	/** Removes `key`. Resolves `true` if it had a value. */
	delete(key: string): Promise<boolean>;
}
