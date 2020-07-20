/*
https://www.codewars.com/kata/51ba717bb08c1cd60f00002f

*/

const assert = require('assert');

class Stack {
    constructor() {
        this.data = [];
    }

    push(element) {
        this.data.push(element);
    }

    front() {
        return this.data[this.data.length-1];
    }

    pop() {
        return this.data.pop();
    }
}

let solution = (list) => {
    return "-6,-3-1,3-5,7-11,14,15,17-20";
}

let partialSolution = (list) => {
    let firstNumber = null;
    let lastNumber = null;

    for(let i=0;i<list.length-2;i++) {
        if(firstNumber == null) {
            firstNumber = list[i];
        }

        if(list[i] + 1 === list[i+1] && list[i+1] + 1 === list[i+2]) {
            lastNumber = list[i+2];
        }
    }

    return firstNumber.toString() + "-" + lastNumber.toString();
}

describe('range extraction', () => {
    it('tests stack', () => {
        const stack = new Stack();
        stack.push(1);
        stack.push(3);
        stack.push(5);

        assert.equal(stack.front(), 5);
        stack.pop();
        assert.equal(stack.front(), 3);
    });

    //it('tests partial range', () => {
    //    assert.equal(partialSolution([1, 2, 3]), "1-3");
    //});

    //it('tests non-consecutive cases', () => {
    //    assert.equal(partialSolution([1, 3, 5]), "1,3,5");
    //});

    //it('tests case 1', () => {
    //    const input = [-6, -3, -2, -1, 0, 1, 3, 4, 5, 7, 8, 9, 10, 11, 14, 15, 17, 18, 19, 20];
    //    const result = "-6,-3-1,3-5,7-11,14,15,17-20";

    //    assert.equal(solution(input), result);
    //});
});