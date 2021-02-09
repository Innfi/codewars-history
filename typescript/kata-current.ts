/**
 * https://www.codewars.com/kata/5270f22f862516c686000161
 */

import assert from 'assert';

const toBase64 = (str: string): string => {
    return '';
};

const fromBase64 = (str: string): string => {
    return '';
};

describe('Building blocks', () => {
    const letters: string = 
        'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/';

    it('get index from base64 target letters', () => {
        assert.strictEqual(letters.indexOf('B'), 1);
        assert.strictEqual(letters.indexOf('r'), 43);
    });

    it('convert ascii to byte code', () => {
        const input: string = 'M';

        assert.strictEqual(input.charCodeAt(0).toString(2).padStart(8, '0'), '01001101');
    });

    it('convert byte code to letter', () => {
        const input: string = '01001101';

        assert.strictEqual(String.fromCharCode(parseInt(input, 2)), 'M');
    });
});

describe('Base64', () => {
    //it('simple case', () => {
    //    assert.strictEqual(toBase64('MAN'), 'TWFu');
    //});

    //it('test case', () => {
    //    assert.strictEqual(toBase64('this is a string!!'), 'dGhpcyBpcyBhIHN0cmluZyEh');
    //    assert.strictEqual(fromBase64('dGhpcyBpcyBhIHN0cmluZyEh'), 'this is a string!!');
    //});
});
