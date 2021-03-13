/**
 * https://www.codewars.com/kata/5259b20d6021e9e14c0010d4
 */

import assert from 'assert';


const reverse = (str: string): string => {
    return str.split(' ')
        .map((value: string) => value.split('').reverse().join(''))
        .join(' ');
};

describe('Building blocks', () => {
    it('try reverse', () => {
        assert.strictEqual('reverse'.split('').reverse().join(''), 'esrever');
    });

    it('try reverse with space', () => {
        const result: string[] = 'double  spaces'.split(' ');
        result.forEach((value: string) => console.log(value));
    });
});

describe('reverse words', () => {
    it('test case', () => {
        assert.strictEqual(reverse('double  spaces'), 'elbuod  secaps');
    });
});