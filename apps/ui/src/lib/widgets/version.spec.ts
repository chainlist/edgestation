import { describe, expect, it } from 'vitest';
import { isNewer, versionProblem } from './version';

describe('versionProblem', () => {
	it.each([
		['0.1.0', '0.1.0'],
		['0.2.0', '0.1.9'],
		['1.0.0', '0.9.9'],
		['0.10.0', '0.9.0'],
		['0.2.0-beta.1', '0.2.0']
	])('lets app %s run a widget that needs %s', (app, need) => {
		expect(versionProblem(app, need)).toBeUndefined();
	});

	it.each([
		['0.1.0', '0.1.1'],
		['0.9.0', '0.10.0'],
		['0.2.5', '1.0.0']
	])('stops app %s from running a widget that needs %s', (app, need) => {
		expect(versionProblem(app, need)).toBe(
			`it needs EdgeStation ${need} or later, and this is ${app}`
		);
	});

	it('rejects a minAppVersion that is not a version', () => {
		expect(versionProblem('0.1.0', '>=0.1')).toBe(
			'its minAppVersion ">=0.1" is not a version like 0.1.0'
		);
	});
});

describe('isNewer', () => {
	it.each([
		['1.2.0', '1.1.9'],
		['0.10.0', '0.9.0'],
		['2.0.0', '1.99.99'],
		['1.0.0', 'not a version']
	])('finds %s newer than %s', (candidate, current) => {
		expect(isNewer(candidate, current)).toBe(true);
	});

	it.each([
		['1.0.0', '1.0.0'],
		['1.0.0', '1.0.1'],
		['soon', '1.0.0']
	])('does not find %s newer than %s', (candidate, current) => {
		expect(isNewer(candidate, current)).toBe(false);
	});
});
