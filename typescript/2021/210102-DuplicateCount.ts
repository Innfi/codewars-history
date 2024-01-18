/**
https://www.codewars.com/kata/54bf1c2cd5b56cc47f0007a1
 */

import assert from 'assert';

type LetterDict = { [id: string]: number };

const duplicateCount = (text: string): number => {
    const letterDict: LetterDict = {};

    text.toLowerCase().split('').map((value: string) => {
        if(letterDict[value] === undefined) letterDict[value] = 1;
        else letterDict[value] += 1;
    });

    let result = 0;
    for(let key in letterDict) {
        if(letterDict[key] >= 2) result += 1;
    }

    return result;
};

describe('Building blocks', () => {
    it('string manipulation', () => {
        const testInput = 'abcde';
        const first: number = testInput.indexOf('a');
        const last: number = testInput.lastIndexOf('a');

        assert.strictEqual(first, last);
    });

    it('toLower to number included string', () => {
        const testInput = 'asdf123GNRT';

        assert.strictEqual(testInput.toLowerCase(), 'asdf123gnrt');
    });

    it('map letters to dict', () => {
        const letterDict: LetterDict = {};
        const testInput = 'invisibilities';

        testInput.split('').map((value: string) => {
            if(letterDict[value] === undefined) letterDict[value] = 1;
            else letterDict[value] += 1;
        });

        assert.strictEqual(letterDict['i'], 6);
    });
});

describe('DuplicateCount', () => {
    it('simple case', () => {
        assert.strictEqual(duplicateCount('abcde'), 0);
        assert.strictEqual(duplicateCount('aabbcde'), 2);
    });

    it('general cases', () => {
        assert.strictEqual(duplicateCount('aabBcde'), 2);
        assert.strictEqual(duplicateCount('Indivisibility'), 1);
        assert.strictEqual(duplicateCount('Indivisibilities'), 2);
    });
});
