/**
 https://www.codewars.com/kata/51b66044bce5799a7f000003
 */


const assert = require('assert');

class RomanNumerals {
    static toRoman = (arabic) => {
        return this.toRomanUnder10(arabic);
    }

    static toRomanUnder10 = (arabic) => {
        if(arabic < 4) return this.toRomanSequence1(arabic);
        if(arabic <= 8) return this.toRomanSeqence5(arabic);
        return this.toRomanSeqence10(arabic);
    }

    static toRomanSequence1 = (arabic) => {
        return 'I'.repeat(arabic);
    }

    static toRomanSeqence5 = (arabic) => {
        if(arabic -5 == -1) return 'IV';
        return 'V' + this.toRomanSequence1(arabic -5);
    }

    static toRomanSeqence10 = (arabic) => {
        //FIXME
        return 'IX';
    }
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

    //it('tests arabic to roman', () => {
    //    assert.equal(RomanNumerals.toRoman(1000), 'M')
    //    assert.equal(RomanNumerals.toRoman(999), "CMXCIX")
    //    assert.equal(RomanNumerals.toRoman(4), 'IV')
    //    assert.equal(RomanNumerals.toRoman(1), 'I')
    //    assert.equal(RomanNumerals.toRoman(1991), 'MCMXCI')
    //    assert.equal(RomanNumerals.toRoman(2006), 'MMVI')
    //    assert.equal(RomanNumerals.toRoman(2020), 'MMXX')
    //});

    //it('tests roman to arabic', () => {
    //    assert.equal(RomanNumerals.fromRoman('XXI'), 21)
    //    assert.equal(RomanNumerals.fromRoman('I'), 1)
    //    assert.equal(RomanNumerals.fromRoman('III'), 3)
    //    assert.equal(RomanNumerals.fromRoman('IV'), 4)
    //    assert.equal(RomanNumerals.fromRoman('MMVII'), 2007)
    //    assert.equal(RomanNumerals.fromRoman('MDCLXIX'), 1669)
    //});
});