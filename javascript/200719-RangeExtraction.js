/*
https://www.codewars.com/kata/51ba717bb08c1cd60f00002f

one of the optimal solutions was : 

function solution(list){
   return list
     .reduce((acc, n, i, arr) => {
       if(n !== arr[i - 1] + 1) {
         acc.push([n]);
       } else {
         acc[acc.length - 1].push(n);
       }
       return acc;
     }, [])
     .map(arr => arr.length > 2 ? `${arr[0]}-${arr[arr.length-1]}` : arr)
     .join(',');
}

*/

const assert = require('assert');

class RangeQueue {
    constructor() {
        this.data = [];
    }

    enque(element) {
        this.data.push(element);
    }

    start() {
        return this.data[0];
    }

    end() {
        return this.data[this.data.length-1];
    }

    empty() {
        return this.data.length <= 0;
    }

    clear() {
        this.data = [];
    }

    isContiguous(element) {
        return this.end() + 1 === element;
    }

    toString() {
        if(this.data.length >= 3) {
            return this.start().toString() + "-" + this.end().toString() + ",";
        } 

        let resultString = "";
        this.data.forEach((queueElement) => {
            resultString += queueElement.toString() + ",";
        });

        return resultString;
    }
}

let solution = (list) => {
    let resultString = "";
    const queue = new RangeQueue();

    list.forEach((element) => {
        if(queue.empty() || queue.isContiguous(element) == true) {
            queue.enque(element);
            return;
        }

        resultString += queue.toString();
        
        queue.clear();
        queue.enque(element);
    });

    return (resultString + queue.toString()).slice(0, -1);
}

describe('range extraction', () => {
    it('tests queue start', () => {
        const queue = new RangeQueue();
        queue.enque(1);
        queue.enque(2);
        assert.equal(queue.start(), 1);

        queue.enque(3);
        assert.equal(queue.start(), 1);
    });

    it('tests queue end', () => {
        const queue = new RangeQueue();
        queue.enque(1);
        queue.enque(2);
        assert.equal(queue.end(), 2);

        queue.enque(3);
        assert.equal(queue.end(), 3);
    });

    it('tests queue empty', () => {
        const queue = new RangeQueue();
        assert.equal(queue.empty(), true);
        queue.enque(1);
        assert.equal(queue.empty(), false);
    });

    it('tests queue clear', () => {
        const queue = new RangeQueue();
        queue.enque(1);
        queue.enque(2);
        queue.enque(3);
        queue.clear();
        assert.equal(queue.empty(), true);
    });

    it('tests queue isContiguous', () => {
        const queue = new RangeQueue();
        queue.enque(1);
        assert.equal(queue.isContiguous(2), true);
        assert.equal(queue.isContiguous(5), false);

        queue.enque(2);
        assert.equal(queue.isContiguous(2), false);
        assert.equal(queue.isContiguous(3), true);
    });

    it('tests queue toString for contiguous elements', () => {
        const queue = new RangeQueue();
        queue.enque(1);
        queue.enque(2);
        queue.enque(3);

        assert.equal(queue.toString(), "1-3,");
    });

    it('tests queue toString for not contiguous enough elements case', () => {
        const queue = new RangeQueue();
        queue.enque(9);
        queue.enque(10);

        assert.equal(queue.toString(), "9,10,");
    });

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