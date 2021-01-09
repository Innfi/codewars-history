/**
https://www.codewars.com/kata/54da5a58ea159efa38000836

one of the optimal solutions is: 

export const findOdd = (items: number[]): number => {
  const uniqueItems = Array.from(new Set(items))
  
  for (const uniqueItem of uniqueItems) {
    const numberOccurences = items.filter(item => item === uniqueItem).length
    if (isOdd(numberOccurences)) return uniqueItem
  }
  
  throw new Error('none found')
}

function isOdd(num: number): boolean {
  return num % 2 === 1
}

*/

import assert from 'assert';

type LetterDict = { [id: string]: number };

const findOdd = (numbers: number[]): number => {
    const dict: LetterDict = {};

    numbers.forEach((value: number) => {
        if(dict[value.toString()] === undefined) dict[value.toString()] = 1;
        else dict[value.toString()] += 1;
    });

    for(let key in dict) {
        const modulo: number = dict[key] % 2;
        if(modulo === 1 || modulo === -1) {
            return Number.parseInt(key);
        }
    }

    return undefined;
};

describe('Building blocks', () => {
    const numbers: number[] = [20,1,-1,2,-2,3,3,5,5,1,2,4,20,4,-1,-2,5];

    it('modulo for minus number', () => {
        assert.strictEqual(-2 % 2, -0);
        assert.strictEqual(-3 % 2, -1);
    });

    it('iterate array', () => {
        numbers.sort();

        let current: number = numbers[0];
        let sum: number = current;

        for(let i=1;i<numbers.length; i++) {
            if(current === numbers[i]) {
                sum += numbers[i];
            } else {
                if(sum % 2 === 1) break;
                if(sum % 2 === -1) break;

                current = numbers[i];
                sum = numbers[i];
            }
        }

        assert.strictEqual(current, 5);
    });

    it('using dictionary', () => {
        const dict: LetterDict = {};

        numbers.forEach((value: number) => {
            if(dict[value.toString()] === undefined) dict[value.toString()] = 1;
            else dict[value.toString()] += 1;
        });

        let result: number = undefined;
        for(let key in dict) {
            const modulo: number = dict[key] % 2;
            if(modulo === 1 || modulo === -1) {
                result = Number.parseInt(key);
                break;
            }
        }
       
        assert.strictEqual(result, 5);
    });
});

describe('Odd number', () => {
    it('test case', () => {
       assert.strictEqual(findOdd([20,1,-1,2,-2,3,3,5,5,1,2,4,20,4,-1,-2,5]), 5);
       assert.strictEqual(findOdd([1,1,2,-2,5,2,4,4,-1,-2,5]), -1);
       assert.strictEqual(findOdd([20,1,1,2,2,3,3,5,5,4,20,4,5]), 5);
       assert.strictEqual(findOdd([10]), 10);
       assert.strictEqual(findOdd([1,1,1,1,1,1,10,1,1,1,1]), 10);
       assert.strictEqual(findOdd([5,4,3,2,1,5,4,3,2,10,10]), 1);
    });
});
