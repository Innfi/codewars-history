/**
 * https://www.codewars.com/kata/58ad317d1541651a740000c5
 */

const assert = require('assert'); 


const middlePermutation = (input) => {
    const sortedInput = input.split('').sort().join('');
    const { start, remain } = toCenterString(sortedInput);

    if(sortedInput.length % 2 === 0) {
        return start + remain.split('').reverse().join('');
    }

    return start + middlePermutation(remain);
};

const toCenterString = (input) => {
    const startLetter = input[toMiddleIndex(input)];
    const remainString = input.replace(startLetter, '');

    return {
        start: startLetter,
        remain: remainString
    };
};

const toMiddleIndex = (input) => {
    const halfLen = parseInt(input.length / 2);
    if(input.length % 2 === 1) return halfLen;

    return halfLen-1;
};

const toPermLen = (input) => {
    let length = input.length;
    let permLen = 1;
    while(length > 1) permLen *= length--;

    return permLen;
};

describe('Building Blocks', () => {
    it('prints permlen', () => {
        assert.equal(toPermLen('cegt'), 24);
        assert.equal(toPermLen('abcde'), 120);
    });

    it('string replace', () => {
        const input4 = 'abcd';
        const startLetter = input4[input4.length/2-1];
        assert.equal(startLetter, 'b');
        const remain = input4.replace(startLetter, '');
        assert.equal(remain, 'acd');
    });

    it('toMiddleIndex', () => {
        assert.equal(toMiddleIndex('abcd'), 1);
        assert.equal(toMiddleIndex('abcde'), 2);
    });

    it('toCenterString odd length', () => {
        const {start, remain} = toCenterString('abcde');
        assert.equal(start, 'c');
        assert.equal(remain, 'abde');
    });

    it('toCenterString even length', () => {
        const {start, remain} = toCenterString('abcdef');
        assert.equal(start, 'c');
        assert.equal(remain, 'abdef');
    });

    it('string sort', () => {
        assert.equal('adbc'.split('').sort().join(''), 'abcd');
    });

    it('string reverse', () => {
        assert.equal('abcd'.split('').reverse().join(''), 'dcba');
    });
});

describe('MiddlePermutation', () => {
    it('runs test case', () => {
        assert.equal(middlePermutation('abc'), 'bac');
        assert.equal(middlePermutation('abcd'), 'bdca');
        assert.equal(middlePermutation('abcdx'), 'cbxda');
        assert.equal(middlePermutation('abcdxg'), 'cxgdba');
        assert.equal(middlePermutation('abcdxgz'), 'dczxgba');
    });   
});