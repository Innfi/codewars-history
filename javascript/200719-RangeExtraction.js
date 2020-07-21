/*
https://www.codewars.com/kata/51ba717bb08c1cd60f00002f

*/

const assert = require('assert');

let solution = (list) => {
    let resultString = "";
    let queue = [];

    list.forEach((element) => {
        if(queue.length <= 0 || queue[queue.length-1] + 1 === element) {
            queue.push(element);
            return;
        }

        if(queue.length >= 3) {
            resultString += queue[0].toString() + "-" + queue[queue.length-1].toString() + ",";
        } else {
            queue.forEach((queueElement) => {
                resultString += queueElement.toString() + ",";
            });
        }

        queue = [];
        queue.push(element);
    });

    if(queue.length >= 3) {
        resultString += queue[0].toString() + "-" + queue[queue.length-1].toString() + ",";
    } else {
        queue.forEach((queueElement) => {
            resultString += queueElement.toString() + ",";
        });
    }

    return resultString.slice(0, -1);
}

describe('range extraction', () => {
    it('tests partial range', () => {
        assert.equal(solution([1, 2, 3]), "1-3");
    });

    it('tests non-consecutive cases', () => {
        assert.equal(solution([1, 3, 5]), "1,3,5");
    });

    it('tests case 1', () => {
        const input = [-6, -3, -2, -1, 0, 1, 3, 4, 5, 7, 8, 9, 10, 11, 14, 15, 17, 18, 19, 20];
        const result = "-6,-3-1,3-5,7-11,14,15,17-20";

        assert.equal(solution(input), result);
    });
});