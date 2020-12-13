/**
 *  https://www.codewars.com/kata/5583d268479559400d000064
 */

import assert from 'assert';

const binaryToString = (binary: string) => {
    if(binary.length <= 0) return '';
    return (binary.match(/.{1,8}/g) as RegExpMatchArray).map((value: string) => {
        return String.fromCharCode(Number.parseInt(value, 2));
    }).join('');
};


describe('Building blocks', () => {
    it('fromCharCode', () => {
        assert.strictEqual(String.fromCharCode(65), 'A');
    });

    it('split to string with length 8', () => {
        const input: string = '011000010110000101100001';

        const splitted: string[] = input.match(/.{1,8}/g);

        splitted.forEach((value: string) => {
            assert.strictEqual(value.length, 8);
        });
    });

    it('string match exceptions', () => {
        const input: string = '1234567';
        const splitted: string[] = input.match(/.{1,8}/g);
        assert.strictEqual(splitted[0], input);
    });
});

describe('test case', () => {
    it('empty input', () => {
        assert.strictEqual(binaryToString(''), '');
    });

    it('simple case', () => {
        assert.strictEqual(binaryToString('01100001'), 'a');
    });

    it('longer input', () => {
        assert.strictEqual(
            binaryToString('01001011010101000100100001011000010000100101100101000101'), 'KTHXBYE')
    });

    it('numeric', () => {
        assert.strictEqual(binaryToString('00110001001100000011000100110001'), '1011');
    });

    it('special characters', () => {
        assert.strictEqual(binaryToString('001111000011101000101001'), '<:)');
    });
});

