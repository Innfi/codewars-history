/**
 * https://www.codewars.com/kata/526d84b98f428f14a60008da
 */

const assert = require('assert');

class HammeringNumbers {
    static hamming(index) {
        return 1;
    } 

    static IsHammering(number) {
        if(number % 2 !== 0) return false;
        if(number % 3 !== 0) return false;
        if(number % 5 !== 0) return false;

        return true;
    }
}

describe('HammeringNumbers', () => {
    it('checks hammering number', () => {
        let targetNumber = 30;

        assert.equal(HammeringNumbers.IsHammering(30), true);        
    });

    it('has 1 for input 1', () => {
        assert.equal(HammeringNumbers.hamming(1), 1);
    });
});