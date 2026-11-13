import type { WidgetApi } from '@edgestation/sdk';

/** A city from the Open-Meteo geocoding, with the search text that found it. */
export interface Place {
	query: string;
	name: string;
	latitude: number;
	longitude: number;
}

export interface Conditions {
	temperature: number;
	/** WMO weather code. */
	code: number;
	day: boolean;
}

export interface Hour extends Conditions {
	/** Milliseconds since the epoch. */
	time: number;
	/** Chance of rain in percent. `null` if the model gives no value. */
	rain: number | null;
}

export interface Forecast {
	/** The name of the city, in the langauge of the app if Open-Meteo has it. */
	place: string;
	/** The IANA time zone of the city. The widget shows the times in this zone. */
	timeZone: string;
	current: Conditions & {
		feelsLike: number;
		/** Relative humidity in percent. */
		humidity: number;
		windSpeed: number;
		/** The direction that the wind comes from, in degrees. 0 is north. */
		windDirection: number;
	};
	/**
	 * 25 hours from the current hour, so the first and the last are one day apart. The first
	 * hour has the current conditions.
	 */
	hours: Hour[];
	/** Milliseconds since the epoch. `null` if the sun does not rise or set today. */
	today: { sunrise: number | null; sunset: number | null; uv: number | null };
}

interface GeocodingResult {
	name: string;
	latitude: number;
	longitude: number;
	country_code?: string;
	country?: string;
	admin1?: string;
}

/** The fields that `loadForecast` gets from Open-Meteo, with `timeformat=unixtime`. */
interface ForecastResponse {
	timezone: string;
	current: {
		time: number;
		temperature_2m: number;
		apparent_temperature: number;
		relative_humidity_2m: number;
		is_day: number;
		weather_code: number;
		wind_speed_10m: number;
		wind_direction_10m: number;
	};
	hourly: {
		time: number[];
		temperature_2m: number[];
		precipitation_probability: (number | null)[];
		weather_code: number[];
		is_day: number[];
	};
	daily: {
		sunrise: (number | null)[];
		sunset: (number | null)[];
		uv_index_max: (number | null)[];
	};
}

async function json<T>(api: WidgetApi, url: string): Promise<T> {
	const response = await api.fetch(url);
	if (!response.ok) throw new Error(`Open-Meteo answered ${response.status}`);
	return response.json();
}

/**
 * Returns the city that `query` names, or `null` if Open-Meteo does not know it. Text after a
 * comma selects one of the cities with the same name, by country code, country or region
 * (`Paris, Texas`). The placement keeps the result, so the widget looks up a city again only
 * when it's setting changes.
 */
export async function findPlace(api: WidgetApi, query: string): Promise<Place | null> {
	const saved = await api.storage.instance.get<Place>('place');
	if (saved?.query === query) return saved;

	const comma = query.indexOf(',');
	const name = (comma < 0 ? query : query.slice(0, comma)).trim();
	const hint =
		comma < 0
			? ''
			: query
					.slice(comma + 1)
					.trim()
					.toLowerCase();
	const params = new URLSearchParams({
		name,
		count: hint ? '20' : '1',
		language: navigator.language.split('-')[0],
		format: 'json'
	});
	const { results = [] } = await json<{ results?: GeocodingResult[] }>(
		api,
		`https://geocoding-api.open-meteo.com/v1/search?${params}`
	);
	const found = results.find(
		(r) =>
			!hint || [r.country_code, r.country, r.admin1].some((s) => s?.toLowerCase().startsWith(hint))
	);
	if (!found) return null;

	const place = { query, name: found.name, latitude: found.latitude, longitude: found.longitude };
	await api.storage.instance.set('place', place);
	return place;
}

/** Returns the forecast at `place`, in °F and mph if `imperial`, or in °C and km/h. */
export async function loadForecast(
	api: WidgetApi,
	place: Place,
	imperial: boolean
): Promise<Forecast> {
	const params = new URLSearchParams({
		latitude: String(place.latitude),
		longitude: String(place.longitude),
		current:
			'temperature_2m,apparent_temperature,relative_humidity_2m,is_day,weather_code,wind_speed_10m,wind_direction_10m',
		hourly: 'temperature_2m,precipitation_probability,weather_code,is_day',
		daily: 'sunrise,sunset,uv_index_max',
		timezone: 'auto',
		// The hours start at local midnight today, so two days always contain the next 24 hours.
		forecast_days: '2',
		timeformat: 'unixtime'
	});
	if (imperial) {
		params.set('temperature_unit', 'fahrenheit');
		params.set('wind_speed_unit', 'mph');
	}
	const data = await json<ForecastResponse>(
		api,
		`https://api.open-meteo.com/v1/forecast?${params}`
	);
	const { current: c, hourly: h, daily: d } = data;

	const current = {
		temperature: c.temperature_2m,
		feelsLike: c.apparent_temperature,
		humidity: c.relative_humidity_2m,
		windSpeed: c.wind_speed_10m,
		windDirection: c.wind_direction_10m,
		code: c.weather_code,
		day: c.is_day === 1
	};
	const start = Math.max(
		0,
		h.time.findLastIndex((time) => time <= c.time)
	);
	const hours = h.time.slice(start, start + 25).map((time, i) => ({
		time: time * 1000,
		temperature: h.temperature_2m[start + i],
		rain: h.precipitation_probability[start + i],
		code: h.weather_code[start + i],
		day: h.is_day[start + i] === 1
	}));
	// The first hour is now. Show the current conditions, not those at the start of the the hour.
	Object.assign(hours[0], {
		temperature: current.temperature,
		code: current.code,
		day: current.day
	});

	const ms = (time: number | null | undefined) => (time == null ? null : time * 1000);
	return {
		place: place.name,
		timeZone: data.timezone,
		current,
		hours,
		today: { sunrise: ms(d.sunrise[0]), sunset: ms(d.sunset[0]), uv: d.uv_index_max[0] ?? null }
	};
}
