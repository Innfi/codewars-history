/**
 *https://www.codewars.com/kata/55b3425df71c1201a800009c
 */

import assert from 'assert';


describe('Building blocks', () => {
    it('tests split', () => {
        const input = '01|59|59';
        const result = input.split('|');
        assert.strictEqual(result[0], '01');
    });

    it('to seconds', () => {
        const input = '01|59|59';
        const result = input
        .split('|')
        .reverse()
        .map(x => Number.parseInt(x)).reduce(
            (prev: number, curr: number, index: number) => {
                return prev + curr * Math.pow(60, index);
            });

        assert.strictEqual(result, 3600 + (59*60) + 59);
    });
});