/*

https://www.codewars.com/kata/5626b561280a42ecc50000d1

one of the optimal solutions was: 

function filterFunc(n) {
  return `${n}`.split("").map((x, i) => x ** (i+1)).reduce((a, b) => a+b) == n;
}

function *range(a, b) {
  for (var i = a; i <= b; ++i) yield i;
}

function sumDigPow(a, b) {
  return Array.from(range(a, b)).filter(filterFunc);
}
*/

const assert = require('assert');

const dummyResult = [1, 2, 3, 4, 5, 6, 7, 8, 9];

sumDigPow = (start, end) => {
	let resultArray = [];
	
	for(let i=start;i<=end;i++) {
		if(isMagicNumber(i)) resultArray.push(i);
	}

	return resultArray;
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
        assert.deepEqual(sumDigPow(1, 10), dummyResult);
    });

	it('tells a candidate is a magic number', () => {
		assert.equal(isMagicNumber(89), true);
		assert.equal(isMagicNumber(88), false);
	});

	it('tests example case 1', () => {
		assert.deepEqual(sumDigPow(1, 10), [1, 2, 3, 4, 5, 6, 7, 8, 9]);
	});

	it('tests example case 2', () => {
		assert.deepEqual(sumDigPow(90, 100), []);
	});

	it('tests example case 3', () => {
		assert.deepEqual(sumDigPow(1, 100), [1, 2, 3, 4, 5, 6, 7, 8, 9, 89]);
	});
});
