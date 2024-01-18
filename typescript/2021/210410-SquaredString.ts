/**
 * https://www.codewars.com/kata/56dbf59b0a10feb08c000227
 */

import assert from 'assert';


function rot90Counter(input: string): string {
    return toString(doTranspose(doReverse(toMatrix(input))));
}

function diag2Sym(input: string): string {
    return toString(doRevTranspose(toMatrix(input)));
}

function selfieDiag2Counterclock(input: string): string {
    const matrix: string[][] = toMatrix(input);   
    const diag2: string[][] = doRevTranspose(toMatrix(input));
    const counterclock: string[][] = doTranspose(doReverse(toMatrix(input)));

    return Object.keys(matrix[0]).map((value, index: number) => {
        return matrix[index].join('') + '|' + diag2[index].join('') + '|' + counterclock[index].join('');
    }).join('\n');

    //return '';
}

function oper(fct: (s: string) => string, s: string): string {
    return fct(s);
}

const toMatrix = (input: string): string[][] => {
    return input.split('\n').map((value: string) => value.split(''));
};

const doReverse = (input: string[][]): string[][] => {
    return input.map((value: string[]) => value.reverse());
};

const doFlip = (input: string[][]): string[][] => {
    return Object.keys(input[0]).map((value, index: number) => input[input.length-1-index]);
};

const doTranspose = (input: string[][]): string[][] => {
    return Object.keys(input[0]).map((value, index: number) => {
        return input.map((row: string[]) => row[index]);
    });
};

const doRevTranspose = (input: string[][]): string[][] => {
    return Object.keys(input[0]).map((value, index: number) => {
        return input.map((row: string[]) => row[input.length-1-index]).reverse();
    });
};

const toString = (input: string[][]): string => {
    return input.map((value: string[]) => value.join('')).join('\n');
};

describe('Building blocks', () => {
    it('split input into arrays', () => {
        const input: string = 'EcGcXJ\naaygcA\nNgIshN\nyOrCZE\neBEqpm\nNkxCgw';
        assert.strictEqual(input.split('\n')[0], 'EcGcXJ');
    });

    it('oneline matrix', () => {
        const input: string = 'EcGcXJ\naaygcA\nNgIshN\nyOrCZE\neBEqpm\nNkxCgw';
        
    });

    it('array handling', () => {
        const testInput: string[][] = [['a','b'], ['c','d']];
        const testOutput: string[][] = [];
        const newRow: string[] = [];
        newRow.push(testInput[0][0]);
        newRow.push(testInput[1][0]);
        testOutput.push(newRow);
        const newRowNext: string[] = [];
        newRowNext.push(testInput[0][1]);
        newRowNext.push(testInput[1][1]);
        testOutput.push(newRowNext);

        assert.deepStrictEqual(testOutput, [['a','c'], ['b','d']]);
    });

    it('array reverse', () => {
        assert.deepStrictEqual([1, 2, 3].reverse(), [3, 2, 1]);

        const input: string[][] = [['a','b','c'], ['d','e','f'], ['g','h','i']];

        const output: string[][] = input.map((value: string[]) => value.reverse());
        assert.deepStrictEqual(output, [['c','b','a'],['f','e','d'],['i','h','g']]);
    });

    it('array transpose', () => {
        const input: number[][] = [[1,2], [3,4]];
        const output: number[][] = Object.keys(input[0]).map((column: string) => {
            return input.map((row: number[]) => row[column]);
        });

        assert.deepStrictEqual(output, [[1,3],[2,4]]);
    });

    it('array reverse transpose', () => {
        const input: number[][] = [[1,2], [3,4]];
        const output: number[][] = Object.keys(input[0]).reverse().map((column, index: number) => {
            return input.map((row: number[]) => row[input.length-1-index]).reverse();
        });

        assert.deepStrictEqual(output, [[4,2],[3,1]]);
    });

    it('tostring', () => {
        const input: string[][] = [['a','b','c'], ['d','e','f'], ['g','h','i']];
        const output: string = input.map((value: string[]) => value.join('')).join('\n');

        assert.strictEqual(output, 'abc\ndef\nghi');
    });

    it('flip', () => {
        const input: number[][] = [ [1,2,3,4], [5,6,7,8], [9,10,11,12], [13,14,15,16] ];

        const output: number[][] = Object.keys(input[0]).map((value, index: number) => {
            return input[input.length-1-index];
        });

        assert.deepStrictEqual(output, [ [13,14,15,16],[9,10,11,12],[5,6,7,8],[1,2,3,4] ]);
    });
});

describe('Moves in squared string', () => {
    it('rotate 90 counter', () => {
       assert.strictEqual(
          oper(rot90Counter, "EcGcXJ\naaygcA\nNgIshN\nyOrCZE\neBEqpm\nNkxCgw"),
          "JANEmw\nXchZpg\ncgsCqC\nGyIrEx\ncagOBk\nEaNyeN"
        );
    });

    it('diag 2 sym', () => {
        assert.strictEqual(
          oper(diag2Sym, "LmvLyg\nDKELBm\nylJhui\nXRXqHD\nzlisCT\nhPqxYb"),
          "bTDimg\nYCHuBy\nxsqhLL\nqiXJEv\nPlRlKm\nhzXyDL"
        );
    });

    it("Basic tests selfieDiag2Counterclock", () => {
        assert.strictEqual(
          oper(selfieDiag2Counterclock, "NJVGhr\nMObsvw\ntPhCtl\nsoEnhi\nrtQRLK\nzjliWg"),
          "NJVGhr|gKilwr|rwliKg\nMObsvw|WLhtvh|hvthLW\ntPhCtl|iRnCsG|GsCnRi\nsoEnhi|lQEhbV|VbhEQl\nrtQRLK|jtoPOJ|JOPotj\nzjliWg|zrstMN|NMtsrz"
        );
    });
});