/**
 * https://www.codewars.com/kata/526d84b98f428f14a60008da
 */

const assert = require('assert');

class HammeringNumbers {
    static hamming(index) {
        if(index < 7) return index;

        let currentIndex = 7;
        let number = 7;

        while(currentIndex <= index) {
            number++;
            if(this.IsHammering(number) === true) {
                console.log("index: ", currentIndex, "number: ", number);
                currentIndex++;
            }
        }

        return number;
    } 

    static IsHammering(number) {
        while(number %2 === 0) number /= 2;
        while(number %3 === 0) number /= 3;
        while(number %5 === 0) number /= 5;

        return number === 1;
    }
}

describe('HammeringNumbers', () => {
    it('checks hammering number', () => {
        let targetNumber = 30;

        assert.equal(HammeringNumbers.IsHammering(24), true);        
        assert.equal(HammeringNumbers.IsHammering(30), true);        
        assert.equal(HammeringNumbers.IsHammering(20), true);        
    });

    it('has 1 for input 1', () => {
        assert.equal(HammeringNumbers.hamming(1), 1);
    });

    it('has 10 for input 9', () => {
        assert.equal(HammeringNumbers.hamming(19), 32);
    });
});