/**
 * https://www.codewars.com/kata/51fda2d95d6efda45e00004e
 */

const assert = require('assert'); 
const { exception } = require('console');


class User {
    constructor() {
        this.ranks = [-8, -7, -6, -5, -4, -3, -2, -1, 
                       1,  2,  3,  4,  5,  6,  7,  8];
        this.current = 0;
        this.rank = this.ranks[this.current];
        this.progress = 0;
    }

    incProgress(newRank) {
        if(this.toIndex(newRank) === -1) throw exception();

        this.progress += this.toProgress(newRank);
        this.evaulateRank();
    }

    toProgress(newRank) {
        const diff = this.toRankDifference(newRank);

        if(diff > 0) return 10 * diff * diff;
        if(diff === 0) return 3;
        if(diff === -1) return 1;

        return 0;
    }

    toRankDifference(newRank) {
        return this.toIndex(newRank) - this.current;
    }

    evaulateRank() {
        while(this.progress >= 100) {
            this.progress -= 100;
            this.rank = this.ranks[++this.current];
        }

        if(this.rank === this.ranks[this.ranks.length-1]) this.progress = 0;
    }

    toIndex(targetRank) {
        return this.ranks.findIndex(rank => rank === targetRank);
    }

    updateRank(rank) {
        this.rank = rank;
        this.current = this.toIndex(this.rank);
    }
}

describe('User', () => {
    it('has default properties', () => {
        let user = new User();

        assert.equal(user.rank, -8);
        assert.equal(user.progress, 0);
    });

    it('check rank difference', () => {
        let user = new User();

        user.updateRank(-1);
        assert.equal(user.toRankDifference(1), 1);
        assert.equal(user.toRankDifference(-2), -1);

        user.updateRank(1);
        assert.equal(user.toRankDifference(-2), -2);
        assert.equal(user.toRankDifference(-1), -1);
    });

    it('check progress', () => {
        let user = new User();

        assert.equal(user.toProgress(-7), 10);
        assert.equal(user.toProgress(-6), 40);
        assert.equal(user.toProgress(-1), 10 * 7 * 7);
        assert.equal(user.toProgress(1), 10 * 8 * 8);
    });

    it('rank ups 1', () => {
        let user = new User();

        user.incProgress(-7);
        assert.equal(user.progress, 10);
        user.incProgress(-5)
        assert.equal(user.progress, 0);
        assert.equal(user.rank, -7);
    });

    it('incProgress rank 1', () => {
        let user = new User();

        user.incProgress(1);
        assert.equal(user.progress, 40);
        assert.equal(user.rank, -2);
    });

    it('incProgress rank 8', () => {
        let user = new User();

        user.updateRank(-1);
        user.incProgress(8);
        assert.equal(user.progress, 40);
        user.incProgress(5);
        assert.equal(user.progress, 41);
    });

    it('test rank limit', () => {
        let user = new User();

        user.updateRank(6);
        for(let i=0;i<20;i++) user.incProgress(8);

        assert.equal(user.rank, 8);
        assert.equal(user.progress, 0);
    });

});