/**
 * https://www.codewars.com/kata/525f4206b73515bffb000b21
 * 
 * 
 * one of the best solutions was 
 * 
 * function add (a, b) {
   var res = '', c = 0
   a = a.split('')
   b = b.split('')
     while (a.length || b.length || c) {
       c += ~~a.pop() + ~~b.pop()
       res = c % 10 + res
       c = c > 9
     }
     return res
   }
 */

const assert = require('assert'); 

let add = (a, b) => {
    const reversedA = toReversedNumber(a);
    const reversedB = toReversedNumber(b);

    if(reversedA.length > reversedB.length) return toNumberStrings(combineNumbers(reversedA, reversedB)); 

    return toNumberStrings(combineNumbers(reversedB, reversedA));
}

let toReversedNumber = (input) => {
    return input.split('').reverse().map(x => Number(x));
}

let combineNumbers = (large, small) => {
    const count = large.length-small.length;
    for(let i=0;i<count;i++) small.push(0);

    let result = [];
    let remainder = 0;
    for(let i=0;i<large.length;i++) {
        let sum = large[i] + small[i] + remainder;

        if(sum >= 10) {
            remainder = 1;
            sum -= 10;
        } else {
            remainder = 0;
        }

        result.push(sum);
    }

    if(remainder > 0) result.push(remainder);

    return result;
} 

let toNumberStrings = (input) => {
    return input.reverse().reduce((acc, cur) => acc.toString() + cur.toString());
}


describe('build blocks', () => {
    it('splits large number strings', () => {
        const largeNumber = '12353422134245265346345134';
        const numberArray = largeNumber.split('');

        assert.equal(numberArray.length, largeNumber.length);
    });

    it('reverse array', () => {
        const numberString = '1234';
        const reversedArray = numberString.split('').reverse();

        assert.equal(reversedArray[0], numberString[numberString.length-1]);
    });

    it('map array', () => {
        const numberString = '1234';
        const mappedNumbers = numberString.split('').map(x => Number(x));

        assert.equal(mappedNumbers[0], Number(numberString[0]));
    });

    it('increments array element: equal length', () => {
        const numbers1 = [ 1, 2, 3, 4 ];
        const numbers2 = [ 4, 3, 2, 1 ];

        let result = [];
        for(let i=0;i<numbers1.length;i++) result.push(numbers1[i] + numbers2[i]);

        assert.deepEqual(result, [5, 5, 5, 5]);
    });

    it('combine two number arrays', () => {
        const numbers1 = [1, 2, 3, 4];
        const numbers2 = [7, 2];

        assert.deepEqual(combineNumbers(numbers1, numbers2), [8, 4, 3, 4]);
    });

    it('combine two number arrays with ascending', () => {
        const numbers1 = [9, 9, 9, 4];
        const numbers2 = [8, 8];
        const result = combineNumbers(numbers1, numbers2);
        assert.deepEqual(result, [7, 8, 0, 5]);
    });

    it('combind two number arrays with additional element', () => {

        const numbers1 = [9, 9, 9, 4];
        const numbers2 = [9, 9, 9, 9];
        const result = combineNumbers(numbers1, numbers2);
        assert.deepEqual(result, [8, 9, 9, 4, 1]);
    });

    it('array reduce', () => {
        const result = [1, 2, 3, 4].reduce((acc, cur) => {
            return acc.toString() + cur.toString();
        });
        assert.equal(result, '1234');
    });
});


describe('Big numbers', () => {
    it('tests normal cases', () => {
        assert.equal(add('1', '2'), '3');
        assert.equal(add('123', '456'), '579');
        assert.equal(add('888', '22'), '910');
    });

    it('tests edge cases', () => {
        assert.equal(add('63829983432984289347293874', '90938498237058927340892374089'), 
            '91002328220491911630239667963');
    });
});
