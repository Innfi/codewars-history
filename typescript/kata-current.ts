/**
 * https://www.codewars.com/kata/545434090294935e7d0010ab
 */

import assert from 'assert';


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
  targetTable: any[] = [];
  projection: Function;

  select(...projection: Function[]): QueryPlan { //need orderBy, groupBy, etc
    if(projection.length === 1) this.projection = projection[0];

    return this;
  } 

  from(...arg: any): QueryPlan { this.targetTable = arg;

    return this;
  }

  execute(): any[] { 
    this.targetTable.map((value: any) => {
       
    });
  }
}

function query(): QueryPlan  { return new QueryPlan();
};

function profession(person: any) { return person.profession;
}

describe('Building blocks', () => {
});

describe('Functional sql', () => { 
  it('simple array', () => { let input: number[] = [1, 2, 3];
      assert.deepStrictEqual(query().select().from(input).execute(), [1, 2, 3]);
      assert.deepStrictEqual(query().from(input).select().execute(), [1, 2, 3]);
  });

  it('object array', () => { //SELECT * FROM persons
    assert.deepStrictEqual(query().select().from(persons).execute(), persons);
  });

  it('apply parameters for select', () => { const output: string[] = ["teacher","teacher","teacher","scientific","scientific","scientific","politician"];
    assert.deepStrictEqual(query().select(profession).from(persons).execute(), output);
  });
});