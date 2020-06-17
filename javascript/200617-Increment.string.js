/*

https://www.codewars.com/kata/54a91a4883a7de5d7800009c

one of the optimal solutions was: 

const incrementString = s => s.replace(/[0-8]?9*$/, m => String(++m));

*/

const assert = require('assert');

incrementString = (input) => {
    let alphaString = input.match(/[a-zA-Z]+/);
    let numberString = '';

    if(alphaString === null) {
        alphaString = '';
        numberString = input;
    } else {
        numberString = input.split(alphaString).pop();
    }

    if(numberString === '') return input + '1';

    let padding = numberString.length;
    const zeroFilled = '0'.repeat(padding);
    const incrementOrig = (Number(numberString)+1).toString();

    if(padding < incrementOrig.length) padding = incrementOrig.length;

    return alphaString + (zeroFilled + incrementOrig).slice(-padding);
}

describe('testing incrementString', () => {

    it('string without number: returns with +1', () => {
        assert.equal(incrementString('foo'), 'foo1');
    });

    it('regular expression with normal numbers', () => {
        const testInput = 'foobar23';
        assert.equal(testInput.match(/\d+/), '23');
    });

    it('regular expression with 0 digits', () => {
        const testInput = 'foobar099';
        assert.equal(testInput.match(/\d+/), '099');
    });

    it('regular expression with characters', () => {
        const testInput = 'foobar11';
        assert.equal(testInput.match(/[a-zA-Z]+/), 'foobar');
    });

    it('get number string length', () => {
        const testInput = 'foobar098';
        const alphaString = testInput.match(/[a-zA-Z]+/);
        const numberString = testInput.split(alphaString).pop();

        assert.equal(numberString, '098');
        assert.equal(numberString.length, 3);
    });

    it('increment numberString', () => {
        const testInput = 'foobar098';
        const alphaString = testInput.match(/[a-zA-Z]+/);
        const numberString = testInput.split(alphaString).pop();

        const zeroFilled = '0'.repeat(numberString.length);
        assert.equal(zeroFilled, '000');

        const testResult = (zeroFilled + (Number(numberString)+1))
            .slice(-zeroFilled.length);

        assert.equal(testResult, '099');
    });

    it('increment numberString 2', () => {
        const testInput = 'foobar099';
        const alphaString = testInput.match(/[a-zA-Z]+/);
        const numberString = testInput.split(alphaString).pop();

        const zeroFilled = '0'.repeat(numberString.length);
        assert.equal(zeroFilled, '000');

        const testResult = (zeroFilled + (Number(numberString)+1))
            .slice(-zeroFilled.length);

        assert.equal(testResult, '100');
    });

    it('numberString without number', () => {
        const testInput = 'foobar';
        const alphaString = testInput.match(/[a-zA-Z]+/);
        const numberString = testInput.split(alphaString).pop();

        assert.equal(numberString, '');
    });

    it('incrementString: input without number', () => {
        assert.equal(incrementString('foobar'), 'foobar1');
    });

    it('regular expression: match not found', () => {
        const testInput = '2';
        const alphaString = testInput.match(/[a-zA-Z]+/);

        assert.equal(alphaString, null);
    });

    it('empty string', () => {
        assert.equal(incrementString(''), '1');
    });

    it('string with number 1', () => {
        assert.equal(incrementString('foobar23'), 'foobar24');
    });

    it('string with number 2', () => {
        assert.equal(incrementString('foobar099'), 'foobar100');
    });

    it('string with number 3', () => {
        assert.equal(incrementString('foobar098'), 'foobar099');
    });

    it('string with number 4', () => {
        assert.equal(incrementString('foobar99'), 'foobar100');
    });

    it('string with number 5', () => {
        assert.equal(incrementString('foobar9'), 'foobar10');
    });

    it('only number 1', () => {
        assert.equal(incrementString('2'), '3');
    });

    it('only number 2', () => {
        assert.equal(incrementString('010'), '011');
    });

    it('only number 3', () => {
        assert.equal(incrementString('009'), '010');
    });
});
