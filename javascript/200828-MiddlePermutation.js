/**
 * https://www.codewars.com/kata/58ad317d1541651a740000c5
 */

const assert = require('assert'); 


const middlePermutation = (input) => {
    const permLen = toPermLen(input);
};

const toPermLen = (input) => {
    let length = input.length;
    let permLen = 1;
    while(length > 1) permLen *= length--;

    return permLen;
};

const toCenterString = (input) => {
    const startLetter = input[toMiddleIndex(input)];
    const remain = input.replace(startLetter, '');

    console.log('start: ', startLetter);
    console.log('remain: ', remain);

    return startLetter + remain;
};

const toMiddleIndex = (input) => {
    const halfLen = parseInt(input.length / 2);
    if(input.length % 2 === 1) return halfLen;

    return halfLen-1;
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

    it('toCenterString', () => {
        assert.equal(toCenterString('abcd'), 'bacd');
        assert.equal(toCenterString('abcde'), 'cabde');
        assert.equal(toCenterString('abcdefg'), 'dabcefg');
    });
});

//describe('MiddlePermutation', () => {
//    it('runs test case', () => {
//        assert.equal(middlePermutation('abc'), 'bac');
//        assert.equal(middlePermutation('abcd'), 'bdca');
//        assert.equal(middlePermutation('abcdx'), 'cbxda');
//        assert.equal(middlePermutation('abcdxg'), 'cxgdba');
//        assert.equal(middlePermutation('abcdxgz'), 'dczxgba');
//    });   
//});