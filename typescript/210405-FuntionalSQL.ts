/**
 * https://www.codewars.com/kata/545434090294935e7d0010ab
 */

import assert from 'assert';
import { exception } from 'console';


var persons = [
    {name: 'Peter', profession: 'teacher', age: 20, maritalStatus: 'married'},
    {name: 'Michael', profession: 'teacher', age: 50, maritalStatus: 'single'},
    {name: 'Peter', profession: 'teacher', age: 20, maritalStatus: 'married'},
    {name: 'Anna', profession: 'scientific', age: 20, maritalStatus: 'married'},
    {name: 'Rose', profession: 'scientific', age: 50, maritalStatus: 'married'},
    {name: 'Anna', profession: 'scientific', age: 20, maritalStatus: 'single'},
    {name: 'Anna', profession: 'politician', age: 50, maritalStatus: 'married'}
];

class QueryPlan { 
    sources: any[] = undefined;
    projection: Function = undefined;
    filter: Function = undefined;
    groupFunc: Function = undefined;

    select(...projection: Function[]): QueryPlan { 
        if(this.projection !== undefined) throw new exception("Duplicate SELECT");
        if(projection.length === 0) this.projection = (value: any) => value;
        if(projection.length === 1) this.projection = projection[0] as Function;
        
        return this;
    } 

    from(...arg: any): QueryPlan { 
        if(this.sources !== undefined) throw new exception("Duplicate FROM");
        this.sources = arg;
        
        return this;
    }

    where(...filter: Function[]): QueryPlan {
        this.filter = filter[0];

        return this;
    }

    groupBy(...groupFunc: Function[]): QueryPlan {
        this.groupFunc = groupFunc[0];

        return this;
    }

    execute(): any[] { 
        if(this.projection === undefined) return [];
        if(this.sources === undefined) return [];
        if(this.filter === undefined) this.filter = (value) => true;

        const result: any[] = [];
        this.sources[0].forEach((value: any) => {
            if(!this.filter.call(this, value)) return;

            const projectionResult = this.projection.call(this, value);
            if(this.groupFunc === undefined) {
              result.push(projectionResult);
              return;
            }

            const groupByKey =  this.groupFunc.call(this, value);
        });

        return result;
    }
}

function query(): QueryPlan  { return new QueryPlan(); };

function profession(person: any) { return person.profession; }
function name(person: any) { return person.name; }

function isTeacher(person: any) { return person.profession === 'teacher'; }

describe('Building blocks', () => {
    it('empty input', () => {
        let input: number[] = [1, 2, 3];
        assert.deepStrictEqual(query().select().execute(), []);
        assert.deepStrictEqual(query().from(input).execute(), []);
    }); 

    it('adding arrays', () => {
        let input: any[] = [];
        input.push('teacher');
        input.push([ { name: 'peter' }, { name: 'michael' }]);
        assert.deepStrictEqual(input,
            [ "teacher", [ { name: 'peter' }, { name: 'michael' } ] ]);
    });
});

describe('Functional sql', () => { 
    it('simple array', () => { 
        let input: number[] = [1, 2, 3];
        assert.deepStrictEqual(query().select().from(input).execute(), [1, 2, 3]);
        assert.deepStrictEqual(query().from(input).select().execute(), [1, 2, 3]);
    });

    it('object array', () => { //SELECT * FROM persons
      assert.deepStrictEqual(query().select().from(persons).execute(), persons);
    });

    it('apply parameters for select', () => { 
      const output: string[] = ["teacher","teacher","teacher","scientific","scientific","scientific","politician"];
      assert.deepStrictEqual(query().select(profession).from(persons).execute(), output);
    });

    it('apply filter1', () => {
        const result: any[] = 
            query().select(profession).from(persons).where(isTeacher).execute();
        assert.deepStrictEqual(result, ["teacher", "teacher", "teacher"]);
    });

    it('apply filter2', () => {
        const result: any[] = 
            query().select().from(persons).where(isTeacher).execute();
    });

    it('apply filter3', () => {
        const result: any[] = 
            query().select(name).from(persons).where(isTeacher).execute();
        assert.deepStrictEqual(result, ["Peter", "Michael", "Peter"]);
    });

    it('execute with groupBy', () => {
      const result: any [] = 
        query().select(name).from(persons).groupBy(profession).execute();
      console.log('result: ', result);
    });
});
