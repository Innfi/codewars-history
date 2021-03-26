/**
 * https://www.codewars.com/kata/515de9ae9dcfc28eb6000001
 */

import assert from 'assert';

const splitStrings = (input) => {
    if(input.length <= 0) return [];
    return (input.length %2 === 1 ? input.concat('_') : input).match(/.{1,2}/g);
};

describe('misc', () => {
    it('test1', () => {
        assert.strictEqual(splitStrings('abcde')[2], 'e_');
    });

    it('test null', () => {
        assert.deepStrictEqual(splitStrings(''), []);
    });
});