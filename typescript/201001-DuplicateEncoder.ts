/**
 *  https://www.codewars.com/kata/54b42f9314d9229fd6000d9c
 */

import assert from 'assert';


describe('Building blocks', () => {
    it('get character occurrences', () => {
        const testString: string = 'qawaeara';

        const count: number = (testString.match(/a/g) || []).length;

        assert.strictEqual(count, 4);
    });
});

//describe('Duplicate Encoder', () => {
//
//});


