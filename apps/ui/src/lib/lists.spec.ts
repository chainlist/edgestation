import { describe, expect, it } from 'vitest';
import { moveIn } from './lists';

describe('moveIn', () => {
	const list = [{ id: 'a' }, { id: 'b' }, { id: 'c' }];
	it('moves an item by delta', () => {
		expect(moveIn(list, 'a', 2).map((i) => i.id)).toEqual(['b', 'c', 'a']);
	});
	it('ignores moves past either end', () => {
		expect(moveIn(list, 'a', -1)).toBe(list);
	});
});
