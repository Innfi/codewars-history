/**
 * https://www.codewars.com/kata/5270f22f862516c686000161
 */

import assert from 'assert';


const toBase64 = (str: string): string => {
    const letters: string = 
        'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/';

    return str
        .split('')
        .map((value: string) => value.charCodeAt(0).toString(2).padStart(8, '0'))
        .join('')
        .match(/.{1,6}/g)
        .map((value: string) => letters.charAt(parseInt(value, 2)) )
        .join('');
};

const fromBase64 = (str: string): string => {
    const letters: string = 
        'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/';
    return str.split('=')[0]
        .split('')
        .map((value: string) => letters.indexOf(value).toString(2).padStart(6, '0'))
        .join('')
        .match(/.{1,8}/g)
        .map((value: string, index: number, array: string[]) => {
            return parseInt(value, 2) !== 0 ? String.fromCharCode(parseInt(value, 2)) : '';
        })
        .join('');
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

    it('cut bytes by 6', () => {
        const input: string = 'Man';

        const converted: string = input
        .split('')
        .map((value: string) => value.charCodeAt(0).toString(2).padStart(8, '0'))
        .join('')
        .match(/.{1,6}/g)
        .map((value: string) => letters.charAt(parseInt(value, 2)) )
        .join('');
       
        assert.strictEqual(converted, 'TWFu');
    });

    it('test filter', () => {
        const input: string = 'test==';

        assert.strictEqual(input.split('=')[0], 'test');
    });

    it('reverse', () => {
        const input: string = 'TWFu=';

        const converted = input.split('=')[0]
        .split('')
        .map((value: string) => letters.indexOf(value).toString(2).padStart(6, '0'))
        .join('')
        .match(/.{1,8}/g)
        .map((value: string) => String.fromCharCode(parseInt(value, 2)))
        .join('');

        assert.strictEqual(converted, 'Man');
    });
});

describe('Base64', () => {
    const letters: string = 
        'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/';
    it('simple case', () => {
        assert.strictEqual(toBase64('Man'), 'TWFu');
    });

    it('paddings', () => {
        const input: string = 'Man is distinguished, not only by his reason, but by this singular passion from other animals, which is a lust of the mind, that by a perseverance of delight in the continued and indefatigable generation of knowledge, exceeds the short vehemence of any carnal pleasure.';
        const output: string = 'TWFuIGlzIGRpc3Rpbmd1aXNoZWQsIG5vdCBvbmx5IGJ5IGhpcyByZWFzb24sIGJ1dCBieSB0aGlzIHNpbmd1bGFyIHBhc3Npb24gZnJvbSBvdGhlciBhbmltYWxzLCB3aGljaCBpcyBhIGx1c3Qgb2YgdGhlIG1pbmQsIHRoYXQgYnkgYSBwZXJzZXZlcmFuY2Ugb2YgZGVsaWdodCBpbiB0aGUgY29udGludWVkIGFuZCBpbmRlZmF0aWdhYmxlIGdlbmVyYXRpb24gb2Yga25vd2xlZGdlLCBleGNlZWRzIHRoZSBzaG9ydCB2ZWhlbWVuY2Ugb2YgYW55IGNhcm5hbCBwbGVhc3VyZS4=';

        const converted: string = input
        .split('')
        .map((value: string) => value.charCodeAt(0).toString(2).padStart(8, '0'))
        .join('')
        .match(/.{1,6}/g)
        .map((value: string, index: number, array: string[]) => {
            return letters
            .charAt(parseInt(value.padEnd(6, '0'), 2))
            .concat('='.repeat((6-value.length)/2));
        })
        .join('');

        assert.strictEqual(output, converted);
    });

    it('reverse paddings', () => {
        const output: string = 'Man is distinguished, not only by his reason, but by this singular passion from other animals, which is a lust of the mind, that by a perseverance of delight in the continued and indefatigable generation of knowledge, exceeds the short vehemence of any carnal pleasure.';
        const input: string = 'TWFuIGlzIGRpc3Rpbmd1aXNoZWQsIG5vdCBvbmx5IGJ5IGhpcyByZWFzb24sIGJ1dCBieSB0aGlzIHNpbmd1bGFyIHBhc3Npb24gZnJvbSBvdGhlciBhbmltYWxzLCB3aGljaCBpcyBhIGx1c3Qgb2YgdGhlIG1pbmQsIHRoYXQgYnkgYSBwZXJzZXZlcmFuY2Ugb2YgZGVsaWdodCBpbiB0aGUgY29udGludWVkIGFuZCBpbmRlZmF0aWdhYmxlIGdlbmVyYXRpb24gb2Yga25vd2xlZGdlLCBleGNlZWRzIHRoZSBzaG9ydCB2ZWhlbWVuY2Ugb2YgYW55IGNhcm5hbCBwbGVhc3VyZS4=';

        const converted: string = input.split('=')[0]
        .split('')
        .map((value: string) => letters.indexOf(value).toString(2).padStart(6, '0'))
        .join('')
        .match(/.{1,8}/g)
        .map((value: string, index: number, array: string[]) => {
            return parseInt(value, 2) !== 0 ? String.fromCharCode(parseInt(value, 2)) : '';
        })
        .join('');

        assert.strictEqual(converted, output);
    });

    it('test case', () => {
        assert.strictEqual(toBase64('this is a string!!'), 'dGhpcyBpcyBhIHN0cmluZyEh');
        assert.strictEqual(fromBase64('dGhpcyBpcyBhIHN0cmluZyEh'), 'this is a string!!');
    });
});

