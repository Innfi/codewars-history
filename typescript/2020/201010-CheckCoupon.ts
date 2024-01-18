/**
 * https://www.codewars.com/kata/539de388a540db7fec000642
 */


import assert from 'assert';

const checkCoupon = (enteredCode: string, correctCode: string, 
                currentDate: string, expirationDate: string): boolean => {
    if(enteredCode !== correctCode) return false;
  
    return Date.parse(currentDate) <= Date.parse(expirationDate);
}

describe('Building blocks', () => {
    it('compare date', () => {
        const dateBefore = Date.parse('Oct 10, 2020');
        const dateAfter = Date.parse('Oct 11, 2020');

        assert.strictEqual(dateBefore < dateAfter, true);
    });
});

describe('CheckCoupon', () => {
    it('test case', () => {
        assert.strictEqual(checkCoupon('123','123','September 5, 2014','October 1, 2014'), true);
        assert.strictEqual(checkCoupon('123a','123','September 5, 2014','October 1, 2014'), false);
        assert.strictEqual(checkCoupon('2222','2222','September 5, 2014','September 5, 2014'), true);
        assert.strictEqual(checkCoupon('2222','2222','September 6, 2014','September 5, 2014'), false);
    });
});