/**
 * https://www.codewars.com/kata/5264d2b162488dc400000001
 */

import assert from 'assert';


const spinWords = (input:string): string => {
    return input.trim().split(' ').map(word => {
        if(word.length < 5) return word;

        return word.split('').reverse().join('');
    }).join(' ');
}

describe('Build blocks', () => {
    it('reverse word', () => {
        const word: string = 'abcd';

        assert.strictEqual(word.split('').reverse().join(''), 'dcba');
        assert.strictEqual(word.split('').reverse().join(' '), 'd c b a');
    });

    it('tests map', () => {
        const testWords: string = 'Hey fellow warriors';

        testWords.split(' ').map(token => {
            console.log(token);
        });
    });

    it('tests trim', () => {
        assert.strictEqual('hey '.trim(), 'hey');
    });
});

describe('Spinning Words', () => {
    it('tests normal cases', () => {
        assert.strictEqual(spinWords("Hey fellow warriors"), "Hey wollef sroirraw");
    });

    it('test edge cases', () => {
        assert.strictEqual(spinWords("Hey "), "Hey");
    });
});