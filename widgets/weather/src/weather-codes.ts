import { ICONS } from './icons.ts';

type IconName = keyof typeof ICONS;

/** The icons that change at night, and their night icon. */
const NIGHT: Partial<Record<IconName, IconName>> = { sun: 'moon', cloudSun: 'cloudMoon' };

/** The WMO weather codes that Open-Meteo sends, with the name and the day icon of each. */
const CODES: Record<number, [label: string, icon: IconName]> = {
	0: ['Clear', 'sun'],
	1: ['Mainly clear', 'sun'],
	2: ['Partly cloudy', 'cloudSun'],
	3: ['Overcast', 'cloud'],
	45: ['Fog', 'fog'],
	48: ['Freezing fog', 'fog'],
	51: ['Light drizzle', 'drizzle'],
	53: ['Drizzle', 'drizzle'],
	55: ['Heavy drizzle', 'drizzle'],
	56: ['Freezing drizzle', 'drizzle'],
	57: ['Freezing drizzle', 'drizzle'],
	61: ['Light rain', 'rain'],
	63: ['Rain', 'rain'],
	65: ['Heavy rain', 'rain'],
	66: ['Freezing rain', 'rain'],
	67: ['Freezing rain', 'rain'],
	71: ['Light snow', 'snow'],
	73: ['Snow', 'snow'],
	75: ['Heavy snow', 'snow'],
	77: ['Snow grains', 'snow'],
	80: ['Light showers', 'rain'],
	81: ['Showers', 'rain'],
	82: ['Heavy showers', 'rain'],
	85: ['Snow showers', 'snow'],
	86: ['Heavy snow showers', 'snow'],
	95: ['Thunderstorm', 'storm'],
	96: ['Thunderstorm with hail', 'hail'],
	99: ['Thunderstorm with hail', 'hail']
};

/** Returns the name of weather `code` and the paths of its day or night icon. */
export function condition(code: number, day: boolean) {
	const [label, icon] = CODES[code] ?? ['Unknown', 'cloud'];
	return { label, paths: ICONS[day ? icon : (NIGHT[icon] ?? icon)] };
}
