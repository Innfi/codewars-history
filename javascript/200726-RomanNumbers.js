/**
 https://www.codewars.com/kata/51b66044bce5799a7f000003
 */


const assert = require('assert');

const arabicDict = {
    1000: 'M',
    900: 'CM',
    500: 'D',
    400: 'CD',
    100: 'C',
    90: 'XC',
    50: 'L',
    40: 'XL',
    10: 'X',
    9: 'IX',
    5: 'V',
    4: 'IV',
    1: 'I',
};

const arabicArray = Object.keys(arabicDict);

class RomanNumerals {
    static toRoman = (arabic) => {
        let roman = "";
        let currentIndex = 0;

        while(arabic > 0 && (currentIndex < arabicArray.length)) {
            let arabicNumber = arabicArray[currentIndex];
            let currentSymbol = arabicDict[arabicNumber];

            if(arabicNumber <= arabic) {
                roman += currentSymbol;
                arabic -= arabicNumber;
            } else {
                currentIndex += 1;
            }
        }
       
        return roman;
    }

    //static toArabic = (roman) => {
    //    
    //}
}

describe('Testing Roman number converter class', () => {
    it('tests string.repeat', () => {
        assert.equal('I'.repeat(0), '');
    });

    it('tests arabic to roman numbers < 10', () => {
        assert.equal(RomanNumerals.toRoman(1), 'I');
        assert.equal(RomanNumerals.toRoman(3), 'III');
        assert.equal(RomanNumerals.toRoman(4), 'IV');
        assert.equal(RomanNumerals.toRoman(5), 'V');
        assert.equal(RomanNumerals.toRoman(6), 'VI');
        assert.equal(RomanNumerals.toRoman(9), 'IX');
    });

    it('tests dictionary', () => {
        var dict = {};
        dict[1] = 'I';
        dict[5] = 'V';

        assert.equal(dict[1], 'I');
    });

    it('tests arabic to roman', () => {
        assert.equal(RomanNumerals.toRoman(1000), 'M')
        assert.equal(RomanNumerals.toRoman(999), "CMXCIX")
        assert.equal(RomanNumerals.toRoman(4), 'IV')
        assert.equal(RomanNumerals.toRoman(1), 'I')
        assert.equal(RomanNumerals.toRoman(1991), 'MCMXCI')
        assert.equal(RomanNumerals.toRoman(2006), 'MMVI')
        assert.equal(RomanNumerals.toRoman(2020), 'MMXX')
    });

    //it('tests roman to arabic', () => {
    //    assert.equal(RomanNumerals.fromRoman('XXI'), 21)
    //    assert.equal(RomanNumerals.fromRoman('I'), 1)
    //    assert.equal(RomanNumerals.fromRoman('III'), 3)
    //    assert.equal(RomanNumerals.fromRoman('IV'), 4)
    //    assert.equal(RomanNumerals.fromRoman('MMVII'), 2007)
    //    assert.equal(RomanNumerals.fromRoman('MDCLXIX'), 1669)
    //});
});