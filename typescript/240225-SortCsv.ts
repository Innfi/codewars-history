// https://www.codewars.com/kata/57f7f71a7b992e699400013f

import assert from 'assert';

const sortCsvColumns = (csvFileContent:string) : string => {
  const intermediate: string[][] = [];

  csvFileContent.split('\n')
    .forEach((row: string, index: number) => {
      const col = row.split(';');
      if (index === 0) {
        col.forEach((elem) => intermediate.push([elem]));
        return;
      }

      col.forEach((elem, innerIdx) => intermediate[innerIdx].push(elem));
    });

  intermediate.sort((a, b) => a[0].localeCompare(b[0]));

  let output = [];
  for (let i=0;i<intermediate[0].length;i++) {
    output.push(intermediate.map((temp) => temp[i]).join(';'));
  }

  return output.join('\n');
}

describe('test', () => {
  it('test case', () => {
    const preSorting = "myjinxin2015;raulbc777;smile67;Dentzil;SteffenVogel_79\n"
                   + "17945;10091;10088;3907;10132\n"
                   + "2;12;13;48;11";
    const postSorting = "Dentzil;myjinxin2015;raulbc777;smile67;SteffenVogel_79\n"
                    + "3907;17945;10091;10088;10132\n"
                    + "48;2;12;13;11";

    assert.strictEqual(sortCsvColumns(preSorting), postSorting);
  });
});
