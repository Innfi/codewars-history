/**
 https://www.codewars.com/kata/51b66044bce5799a7f000003
 */


const assert = require('assert');

const numberDict = {
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

const numberArray = Object.keys(numberDict).reverse();

const romanDict = {
    'CM': 900,
    'M': 1000,
    'CD': 400,
    'D': 500,
    'XC': 90,
    'C': 100,
    'XL': 40,
    'L': 50,
    'IX': 9,
    'X': 10,
    'IV': 4,
    'V': 5,
    'I': 1
};

const romanArray = Object.keys(romanDict);

class RomanNumerals {
    static toRoman = (arabic) => {
        let roman = "";
        let currentIndex = 0;

        while(arabic > 0 && (currentIndex < numberArray.length)) {
            let arabicNumber = numberArray[currentIndex];

            if(arabic >= arabicNumber) {
                roman += numberDict[arabicNumber];
                arabic -= arabicNumber;
            } else {
                currentIndex += 1;
            }
        }
       
        return roman;
    }

    static fromRoman = (roman) => {
        let arabic = 0;
        let currentIndex = 0;

        while(roman.length > 0 && (currentIndex < romanArray.length)) {
            const romanLetter = romanArray[currentIndex];
            console.log('roman: ', roman, ', letter: ', romanLetter);

            if(roman.startsWith(romanLetter)) {
                arabic += romanDict[romanLetter];
                roman = roman.slice(romanLetter.length);
                currentIndex = 0;
            } else {
                currentIndex += 1;
            }
        }

        return arabic;
    }
}

describe('Testing Roman number converter class', () => {
    it('tests arabic to roman', () => {
        assert.equal(RomanNumerals.toRoman(1000), 'M');
        assert.equal(RomanNumerals.toRoman(999), "CMXCIX");
        assert.equal(RomanNumerals.toRoman(4), 'IV');
        assert.equal(RomanNumerals.toRoman(1), 'I');
        assert.equal(RomanNumerals.toRoman(1991), 'MCMXCI');
        assert.equal(RomanNumerals.toRoman(2006), 'MMVI');
        assert.equal(RomanNumerals.toRoman(2020), 'MMXX');
    });

    it('tests string startsWith', () => {
        assert.equal('DCLX'.startsWith('DC'), true);
    });

    it('tests roman to arabic', () => {
        assert.equal(RomanNumerals.fromRoman('XXI'), 21);
        assert.equal(RomanNumerals.fromRoman('I'), 1);
        assert.equal(RomanNumerals.fromRoman('III'), 3);
        assert.equal(RomanNumerals.fromRoman('IV'), 4);
        assert.equal(RomanNumerals.fromRoman('MMVII'), 2007);
        assert.equal(RomanNumerals.fromRoman('MDCLXIX'), 1669);
    });
});