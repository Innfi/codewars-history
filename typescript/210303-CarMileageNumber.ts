/**
 * https://www.codewars.com/kata/52c4dd683bfd3b434c000292
 * 
 * one of the best solutions was: 
 * 
 * export function isInteresting(n: number, awesomePhrases: number[]): number {
  return checkInteresting(n, awesomePhrases) ? 2 : 
          checkInteresting(n + 1, awesomePhrases) 
          || checkInteresting(n + 2, awesomePhrases) ? 1 : 0;
}

const checkInteresting = (n: number, awesomePhrases: number[]): boolean => {
  const nArr = n.toString().split('').map(Number);
  
  return nArr.length >= 3 && (
      /^\d0+$/.test(n.toString())
      || /^(\d)\1{2,}$/.test(n.toString())
      || awesomePhrases.includes(n)
      || '1234567890'.includes(n.toString())
      || '9876543210'.includes(n.toString())
      || nArr.slice().reverse().every((e, i) => e == nArr[i])
  );
}
 */

import assert from 'assert';

const isInteresting = (n: number, awesomePhrases: number[]): number => {
    if (n+2 < 100) return 0;

    if(isInterestingWithPadding(n, awesomePhrases, 2) > 0) return 2;
    if(isInterestingWithPadding(n+1, awesomePhrases, 1) > 0) return 1;
    if(isInterestingWithPadding(n+2, awesomePhrases, 1) > 0) return 1;

    return 0;
};

const isInterestingWithPadding = (n: number, awesomePhrases: number[], result: number): number => {
    if (isFollowingZeros(n.toString())) return result;
    if (isSameNumbers(n.toString())) return result;
    if (isIncrementing(n.toString())) return result;
    if (isDecrementing(n.toString())) return result; 
    if (isPalindrome(n.toString())) return result;
    if (isAwesome(n, awesomePhrases)) return result;

    return 0;
};

const isFollowingZeros = (input: string): boolean => {
    return new RegExp('^[1-9]{1}[0]+$').test(input);
};

const isSameNumbers = (input: string): boolean => {
    return new RegExp('^([1-9])\\1\\1+$').test(input);
};

const isIncrementing = (input: string): boolean => {
    let result: boolean = true;
    const splitted: string[] = input.split('');
    if(splitted.length < 3) return;

    splitted.map((value: string) => Number.parseInt(value))
        .map((value: number, index: number, array: number[]) => {
            if (index+1 >= array.length) return;
            if ((value+1)%10 !== array[index+1]) result = false;
        });

    return result;
};

const isDecrementing = (input: string): boolean => {
    let result: boolean = true;
    const splitted: string[] = input.split('');
    if(splitted.length < 3) return;

    splitted.map((value: string) => Number.parseInt(value))
        .map((value: number, index: number, array: number[]) => {
            if (index + 1 === array.length) return;
            if ((value-1)%10 !== array[index+1]) result = false;
        });

    return result;
};

const isPalindrome = (input: string): boolean => {
    const splitted: string[] = input.split('');
    if(splitted.length < 3) return false;

    return input === splitted.reverse().join('');
};

const isAwesome = (input: number, awesomePhrases: number[]): boolean => {
    return awesomePhrases.find(e => e === input) !== undefined;
};

describe('Building blocks', () => {
    it('check digit followed by all zeros', () => {
        const regex = new RegExp('^[1-9]{1}[0]+$');
        assert.strictEqual(regex.test('90000'), true);
        assert.strictEqual(regex.test('90001'), false);
        assert.strictEqual(regex.test('110000'), false);
    });

    it('check same number', () => {
        const regex = new RegExp('^([1-9])\\1+$');
        assert.strictEqual(regex.test('97777'), false);
        assert.strictEqual(regex.test('222'), true);
        assert.strictEqual(regex.test('4444'), true);
        assert.strictEqual(regex.test('00000'), false);
        assert.strictEqual(regex.test('11115'), false);
    });

    it('check incrementing sequence', () => {
        const incrementing: string = '12345';

        let result: boolean = true;
        incrementing.split('')
            .sort()
            .map((value: string) => Number.parseInt(value))
            .map((value: number, index: number, array: number[]) => {
                if (index + 1 === array.length) return;
                if (value+1 !== array[index+1]) result = false;
            });

        assert.strictEqual(result, true);
    });

    it('check decrementing sequence', () => {
        const decrementing: string = '654321';
        const sorted: string = decrementing.split('').sort().reverse().join('');

        assert.strictEqual(decrementing, sorted);
    });

    it('array find', () => {
        const input: number[] = [1, 2, 3];
        assert.strictEqual(input.find(e => e === 2), 2);
        assert.strictEqual(input.find(e => e === 5), undefined);
    });
});

describe('interesting car mileage', () => {
    it('normal cases', () => {
       assert.strictEqual(isInteresting(3, [1337, 256]), 0);
       assert.strictEqual(isInteresting(1336, [1337, 256]), 1);
       assert.strictEqual(isInteresting(1337, [1337, 256]), 2);
       assert.strictEqual(isInteresting(11208, [1337, 256]), 0);
       assert.strictEqual(isInteresting(11209, [1337, 256]), 1);
       assert.strictEqual(isInteresting(11211, [1337, 256]), 2);
       assert.strictEqual(isInteresting(120, []), 1);
       assert.strictEqual(isInteresting(119, []), 1);
       assert.strictEqual(isInteresting(123, []), 2);
       assert.strictEqual(isInteresting(1234, []), 2);
       assert.strictEqual(isInteresting(67890, []), 2);
       assert.strictEqual(isInteresting(98, []), 1);
       assert.strictEqual(isInteresting(99, []), 1);
    });
});