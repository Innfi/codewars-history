/**
 *https://www.codewars.com/kata/5536a85b6ed4ee5a78000035
 */

import assert from 'assert';


const tour = (friends: string[], fTowns: any, distTable: (string|number)[]): 
    number => {
    const dists: { [id: string]: number } = {};
    fTowns.forEach((value: string[]) => {
        const [ friend, town ] = value;
        const townDist: number = 
            distTable[distTable.findIndex(x => x === town)+1] as number;

        dists[friend] = townDist;
    });

    const distArray: number[] = friends.map((value: string) => {                
        return dists[value]? dists[value] : 0;
    });
    const addArray: number[] = [0, ...distArray, 0];

    let sum: number = 0;
    addArray.reduce((prev: number, current: number) => {
        sum += missingLength(current, prev);
        return current;
    });

    return Math.floor(sum);
};

const missingLength = (lhs: number, rhs: number): number => {
    if(lhs === 0) return rhs;
    if(rhs === 0) return lhs;

    return Math.sqrt(Math.pow(lhs, 2) - Math.pow(rhs, 2));
};

describe('Building blocks', () => {
    it('gets missing length of a triangle', () => {
        const x1: number = 3;
        const x3: number = 5;

        assert.strictEqual(missingLength(5, 3), 4);
    });

    it('sample input', () => {
        const testResult = 100 + 
            missingLength(200, 100) + 
            missingLength(250, 200) + 
            missingLength(300, 250) + 300;
        assert.strictEqual(Math.floor(testResult), 889);
    });

    it('playing with array', () => {
        const distTable1 = ["X1", 100.0, "X2", 200.0, "X3", 250.0, "X4", 300.0];

        distTable1.forEach((value: string | number, index: number, 
            array: (string|number)[]) => {
            if(typeof value === "string") {
                //console.log('string: ', value);
                //console.log('number: ', array[index+1]);
            } 
        });
        
        assert.strictEqual(distTable1.findIndex(value => value === 'X3'), 4);
    });

    it('get distance by friend order', () => {
        const friends = ["A1", "A2", "A3", "A4", "A5"];
        const fTowns = [["A1", "X1"], ["A2", "X2"], ["A3", "X3"], ["A4", "X4"]];
        const distTable = ["X1", 100.0, "X2", 200.0, "X3", 250.0, "X4", 300.0];
        const dists: { [id: string]: number } = {};

        fTowns.forEach((value: string[]) => {
            const [ friend, town ] = value;
            const townDist: number = 
                distTable[distTable.findIndex(x => x === town)+1] as number;

            dists[friend] = townDist;
        });

        const distArray: number[] = friends.map((value: string) => {                
            return dists[value]? dists[value] : 0;
        });

        const addArray: number[] = [0, ...distArray, 0];
        let sum: number = 0;
        addArray.reduce((prev: number, current: number) => {
            sum += missingLength(current, prev);
            return current;
        });

        assert.strictEqual(dists['A3'], 250.0);
        assert.strictEqual(dists['A2'], 200.0);
    });

    it('array test', () => {
        const friends1 = ["A1", "A2", "A3", "A4", "A5"];
        const addFriends = [0, ...friends1, 0];

        console.log(addFriends);
    });
});

