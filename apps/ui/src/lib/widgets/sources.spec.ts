import { invoke } from '@tauri-apps/api/core';
import { describe, expect, it, vi } from 'vitest';
import { readSource, sourcedChoices } from './sources';
import type { AudioApp, SelectSetting } from './types';

vi.mock('@tauri-apps/api/core', () => ({ invoke: vi.fn() }));

const setting: SelectSetting = {
	key: 'channel',
	label: 'Channel',
	type: 'select',
	options: [{ value: 'all', label: 'All' }],
	source: 'audioApps'
};

const app = (name: string, program: string | null): AudioApp => ({
	id: name,
	name,
	path: program && `C:\\${program}`,
	program,
	volume: 1,
	muted: false,
	active: false
});

describe('readSource', () => {
	it('lists the audio apps by program, once each, without System sounds', async () => {
		vi.mocked(invoke).mockResolvedValueOnce({
			output: null,
			input: null,
			apps: [
				app('Discord', 'Discord.exe'),
				app('Teams', 'msedgewebview2.exe'),
				app('EdgeStation', 'msedgewebview2.exe'),
				app('System Sounds', null)
			]
		});
		expect(await readSource('audioApps')).toEqual([
			{ value: 'Discord.exe', label: 'Discord' },
			{ value: 'msedgewebview2.exe', label: 'Teams' }
		]);
	});

	it('lists nothing when the list fails or is unknown', async () => {
		vi.mocked(invoke).mockRejectedValueOnce(new Error('no audio'));
		expect(await readSource('audioApps')).toEqual([]);
		expect(await readSource('nope' as SelectSetting['source'] & string)).toEqual([]);
	});
});

describe('sourcedChoices', () => {
	const read = [{ value: 'Discord.exe', label: 'Discord' }];

	it('puts the own options first, then the ones read', () => {
		expect(sourcedChoices(setting, read, 'all')).toEqual([
			{ value: 'all', label: 'All' },
			{ value: 'Discord.exe', label: 'Discord' }
		]);
	});

	it('keeps a saved value that is not running, marked once the list is read', () => {
		expect(sourcedChoices(setting, read, 'Spotify.exe').at(-1)).toEqual({
			value: 'Spotify.exe',
			label: 'Spotify (not running)'
		});
		expect(sourcedChoices(setting, undefined, 'Spotify.exe').at(-1)).toEqual({
			value: 'Spotify.exe',
			label: 'Spotify.exe'
		});
	});
});
