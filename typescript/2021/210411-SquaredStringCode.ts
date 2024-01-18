/**
 * https://www.codewars.com/kata/56fcc393c5957c666900024d
 */

import assert from 'assert';

const code = (s: string): string => {
    if(s.length < 1) return '';
    const size: number = Math.ceil(Math.sqrt(s.length));
    
    return toString(
        doFlip(
            doRevTranspose(
                toMatrix(toPaddedInput(s, size), size)
            ) 
        )
    );
};

const decode = (s: string): string => {
    if(s.length < 1) return '';
    return toRevString(
        doTranspose(
            doReverse(
                toRevMatrix(s)   
            )
        )
    );
};

const toPaddedInput = (input: string, size: number): string => {
    return input.padEnd(size*size, String.fromCharCode(11));
};

const toMatrix = (input: string, size: number): string[][] => {
    return input.match(new RegExp('.{1,' + size + '}', 'g'))!
        .map((value: string) => value.split(''));
};

const toRevMatrix = (input: string): string[][] => {
    return input.split('\n').map((value: string) => value.split(''));
};

const toString = (input: string[][]): string => {
    return input.map((value: string[]) => value.join('')).join('\n');
};

const toRevString = (input: string[][]): string => {
    return input.map((value: string[]) => value.join('')).join('').trim();
};

const doRevTranspose = (input: string[][]): string[][] => {
    return Object.keys(input[0]).map((value, index: number) => {
        return input.map((row: string[]) => row[input.length-1-index]).reverse();
    });
};

const doFlip = (input: string[][]): string[][] => {
    return Object.keys(input[0]).map((value, index: number) => input[input.length-1-index]);
};

const doReverse = (input: string[][]): string[][] => {
    return input.map((value: string[]) => value.reverse());
};

const doTranspose = (input: string[][]): string[][] => {
    return Object.keys(input[0]).map((value, index: number) => {
        return input.map((row: string[]) => row[index]);
    });
};


describe('Building blocks', () => {
    const input: string[][] = [['a','b','c'],['d','e','f'],['g','h','i']];
    const output: string[][] = [['g','d','a'],['h','e','b'],['i','f','c']];
    it('clockwise rotation', () => {
        assert.deepStrictEqual(doFlip(doRevTranspose(input)), output);
    });

    it('counter clockwise rotation', () => {
        assert.deepStrictEqual(doTranspose(doReverse(output)), input);
    });

    it('padding', () => {
        console.log('padding: ', String.fromCharCode(11));
        assert.strictEqual(Math.sqrt(16), 4);
        assert.strictEqual(Math.ceil(Math.sqrt(15)), 4);
        assert.strictEqual(Math.ceil(Math.sqrt(14)), 4);
        assert.strictEqual(Math.ceil(Math.sqrt(13)), 4);
        assert.strictEqual(Math.ceil(Math.sqrt(12)), 4);
        assert.strictEqual(Math.ceil(Math.sqrt(11)), 4);
        assert.strictEqual(Math.ceil(Math.sqrt(10)), 4);
        assert.strictEqual(Math.ceil(Math.sqrt(9)), 3);

        const input: string = 'abcdefghij';
        const blockSize: number = Math.ceil(Math.sqrt(input.length));

        //console.log(input.match(new RegExp('.{1,' + blockSize + '}', 'g')));
        //console.log('blockSize: ', blockSize);
        //const input2: string = input.padEnd(blockSize*blockSize, String.fromCharCode(11));
        //console.log('len: ', input2.length);
    });
});

describe('Code with squared strings', () => {
    it('test case', () => {
        const data1 = "What do you remember? When I looked at his streaky glasses, I wanted " +
                 "to leave him. And before that? He stole those cherries for me at midnight. We were walking " +
                 "in the rain and I loved him. And before that? I saw him coming " +
                 "toward me that time at the picnic, edgy, foreign."

        const data2 = "Some say the world will end in fire, Some say in ice. From what I've tasted of desire " +
                 "I hold with those who favor fire. But if it had to perish twice, I think I know enough of hate" +
                 " To say that for destruction ice Is also great And would suffice."

        const data1Sol =
        "\vctg?.nadr d gdbW\n\v,i    lnis tl eh\n\v mtIAakietboaara\n\veeo nnigsoe st?t\n\vd wsddnh lfls   \n\vgaaa  gtfeoeehWd\n" +
        "\vytrwbI .o rasiho\n\v, d e i rtev,se \n\v t hflnW h e  ny\n\vfhmioo emot Is o\n\voeemrvt eshh tIu\n\vr   eehw eaiwr  \n" +
        "\veptc deea tmaelr\n\viihot  rtc?.naoe\n\vgcamhhre h  tkom\n\vnntiaia meHAeyke\n\v.i ntmiwirend em"

        const data2Sol =
        "fa  h ttrheI ilS\nitifakw   s'irdo\nc cotnihftivce m\neAereocaihree,we\n.n   wedroe . i \n\vdIdT , es t Sls\n\v seoe t.eIaFola\n" +
        "\vw s nIo   srm y\n\voatso  Bwhtoee \n\vulrautpuhoem nt\n\vlsuyghetold sdh\n\vdoc hir  d wa e\n\v  tt niif ohyi \n\vsgihoksfawfa nw\n" +
        "\vuroaf h vi ti o\n\vfent I iotd nfr"

        assert.strictEqual(code(data1), data1Sol);
        assert.strictEqual(code(data2), data2Sol);
        assert.strictEqual(code(""), "");
        assert.strictEqual(decode(data2Sol), data2)
    });
});