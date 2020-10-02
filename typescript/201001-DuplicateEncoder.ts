/**
 *  https://www.codewars.com/kata/54b42f9314d9229fd6000d9c
 */

import assert from 'assert';

const duplicateEncode = (word: string): string {
    let result: string = '';

    for(const letter of word.toLowerCase().split('')) {
        const re = new RegExp(letter, 'g');
        const count: number = (word.match(re) || []).length;

        if(count > 1) result += ')';
        else result += '(';
    }
} 

describe('Building blocks', () => {
    it('get character occurrences', () => {
        const testString: string = 'qawaeara';

        const count: number = (testString.match(/a/g) || []).length;

        assert.strictEqual(count, 4);
    });

    it('bruteforce approach', () => {
        const testString: string = 'qawaeara';

        let result: string = '';

        for(const letter of testString.split('')) {
            const re = new RegExp(letter, 'g');
            const count: number = (testString.match(re) || []).length;

            if(count > 1) result += ')';
            else result += '(';
        }

        assert.strictEqual(result, '()()()()');
    });
});

//describe('Duplicate Encoder', () => {
//
//});


