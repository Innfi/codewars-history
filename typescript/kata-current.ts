/**
 * https://www.codewars.com/kata/52c4dd683bfd3b434c000292
 */

import assert from 'assert';


describe('Building blocks', () => {
    it('check digit followd by all zeros', () => {
        const regex = new RegExp('^[1-9]{1}[0]+$');
        assert.strictEqual(regex.test('90000'), true);
        assert.strictEqual(regex.test('90001'), false);
        assert.strictEqual(regex.test('110000'), false);
    });

    it('check same number', () => {
        const regex = new RegExp('^([1-9])\\1+$');
        assert.strictEqual(regex.test('97777'), false);
        assert.strictEqual(regex.test('222'), true);
        assert.strictEqual(regex.test('4444'), true);
        assert.strictEqual(regex.test('00000'), false);
        assert.strictEqual(regex.test('11115'), false);
    });

    it('check incrementing sequence', () => {
        const incrementing: string = '12345';
    });

    it('check decrementing sequence', () => {

    });

});
