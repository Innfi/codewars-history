// https://www.codewars.com/kata/62c376ce1019024820580309

import assert from 'assert';

export function getCellAddresses(range: string): string[] {
  const [start, end] = range.split(':').map((v) => {
    return {
      col: v.charCodeAt(0),
      row: Number.parseInt(v.substring(1)),
    };
  });

  const output: string[] = [];
  for (let i=start.row;i<=end?.row;i++) {
    for (let j=start.col; j<=end?.col; j++) {
      output.push(`${String.fromCharCode(j)}${i}`);
    }
  }

  return output.length >= 2 ? output : [];
}

describe('test', () => {
  it ('basic cases', () => {
    assert.deepStrictEqual(
      getCellAddresses("A1:A10"), 
      ['A1', 'A2', 'A3', 'A4', 'A5', 'A6', 'A7', 'A8', 'A9', 'A10']
    );

    assert.deepStrictEqual(getCellAddresses("F12:J17"), [
      'F12', 'G12', 'H12', 'I12', 'J12', 
      'F13', 'G13', 'H13', 'I13', 'J13', 
      'F14', 'G14', 'H14', 'I14', 'J14', 
      'F15', 'G15', 'H15', 'I15', 'J15', 
      'F16', 'G16', 'H16', 'I16', 'J16', 
      'F17', 'G17', 'H17', 'I17', 'J17'
    ]);

    assert.deepStrictEqual(getCellAddresses("W118:Z124"), [
      'W118', 'X118', 'Y118', 'Z118',
      'W119', 'X119', 'Y119', 'Z119',
      'W120', 'X120', 'Y120', 'Z120',
      'W121', 'X121', 'Y121', 'Z121',
      'W122', 'X122', 'Y122', 'Z122',
      'W123', 'X123', 'Y123', 'Z123',
      'W124', 'X124', 'Y124', 'Z124'
    ]);
  });

  it ('error case', () => {
    assert.deepStrictEqual(getCellAddresses("C2"), []);
  });
});

