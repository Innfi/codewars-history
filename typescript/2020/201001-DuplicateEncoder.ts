/**
 *  https://www.codewars.com/kata/54b42f9314d9229fd6000d9c
 * 
 *export function duplicateEncode(word: string){
        // ...
        return word
        .toLowerCase()
        .split('')
        .map((a, i, w) => {
          return w.indexOf(a) == w.lastIndexOf(a) ? '(' : ')'
        })
        .join('')
    }
 */

import assert from 'assert';

const duplicateEncode = (word: string): string => {
    let result: string = '';
    interface Dictionary<T> { [key: string]: T; }
    let letterDict: Dictionary<number> = {};

    const wordArray = word.toLowerCase().split('');
    for(const letter of wordArray) {
        if(letterDict[letter] === undefined) letterDict[letter] = 0;

        letterDict[letter] += 1;
    }

    for(const letter of wordArray) {
        if(letterDict[letter] > 1) result += ')';
        else result += '(';
    }

    return result;
}

const duplicateEncodeBackup = (word: string): string => {
    let result: string = '';

    for(const letter of word.toLowerCase().split('')) {
        const re = new RegExp(letter, 'g');
        const count: number = (word.match(re) || []).length;

        if(count > 1) result += ')';
        else result += '(';
    }

    return result;
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

    it('dictionary edge case', () => {
        interface Dictionary<T> { [key: string]: T; }
        let letterDict: Dictionary<number> = {};
       
        assert.strictEqual(letterDict['notexist'], undefined);
    });
});

describe('Duplicate Encoder', () => {
    it('test case', () => {
        assert.strictEqual(duplicateEncode('(( @)'), '))(((');
    });
});


