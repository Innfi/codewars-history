/*

https://www.codewars.com/kata/5626b561280a42ecc50000d1


*/

const assert = require('assert');

const dummyResult = [1, 2, 3, 4, 5, 6, 7, 8, 9];

sumDigPow = (start, end) => {
    return dummyResult;
}

isMagicNumber = (number) => {
	const numberString = number.toString();

	let testNumber = 0;

	for(let i=0;i<numberString.length;i++) {
		testNumber += Math.pow(numberString[i], i+1);
	}

	return testNumber === number;
}


describe('consecutive power numbers', () => {
    it('shaping the function', () => {
        assert.equal(sumDigPow(1, 10), dummyResult);
    });

	it('tells a candidate is a magic number', () => {
		assert.equal(isMagicNumber(89), true);

		assert.equal(isMagicNumber(88), false);
	});
});
