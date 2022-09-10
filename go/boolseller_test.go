// https://www.codewars.com/kata/54dc6f5a224c26032800005c

/*
func StockList(listArt []string, listCat []string) string {
    if len(listArt) == 0 || len(listCat) == 0 {
      return ""
    }

    quantity := make(map[string]int)
    for _, stock := range listArt {
      splitted := strings.Split(stock, " ")
      firstCh := string(splitted[0][0])
      n, _ := strconv.Atoi(splitted[1])
      quantity[firstCh] += n
    }

    result := make([]string, len(listCat))
    for i, code := range listCat {
      result[i] = fmt.Sprintf("(%s : %v)", code, quantity[code])
    }

    return strings.Join(result, " - ")
}
*/

package codewars_history

import (
	"fmt"
	"reflect"
	"strconv"
	"strings"
	"testing"

	"github.com/stretchr/testify/assert"
)

func StockList(listArt []string, listCat []string) string {
	if len(listArt) <= 0 {
		return ""
	}
	if len(listCat) <= 0 {
		return ""
	}

	var dict map[string]int = make(map[string]int)

	for _, bookData := range listArt {
		symbol, count := toKV(bookData)
		dict[symbol] += count
	}

	result := []string{}
	for _, req := range listCat {
		count := dict[req]

		result = append(result, fmt.Sprintf("(%v : %v)", req, count))
	}

	return strings.Join(result[:], " - ")
}

func toKV(input string) (string, int) {
	splitted := strings.Fields(input)
	count, _ := strconv.ParseInt(splitted[1], 10, 32)

	return splitted[0][0:1], int(count)
}

func TestToKV(t *testing.T) {
	symbol, count := toKV("BBAB 10")

	assert.Equal(t, symbol, "B")
	assert.Equal(t, count, 10)
}

func TestStockList(t *testing.T) {
	var b = []string{"BBAR 150", "CDXE 515", "BKWR 250", "BTSQ 890", "DRTY 600"}
	var c = []string{"A", "B", "C", "D"}

	result := StockList(b, c)
	expected := "(A : 0) - (B : 1290) - (C : 515) - (D : 600)"

	assert.Equal(t, result, expected)
}

func TestDictionaryEmptyKey(t *testing.T) {
	var dict map[string]int = make(map[string]int)

	dict["A"] = 20

	assert.Equal(t, dict["B"], 0)
}

func TestDictionary(t *testing.T) {
	var dict map[string]int = make(map[string]int)

	dict["A"] = 20
	assert.Equal(t, dict["A"], 20)

	dict["A"] += 31
	assert.Equal(t, dict["A"], 51)
}

func TestStringToNumber(t *testing.T) {
	input := "150"
	var expected int64 = 150

	intVar, _ := strconv.ParseInt(input, 0, 64)
	assert.Equal(t, intVar, expected)
}

func TestStringField(t *testing.T) {
	input := "BBAR 150"
	splitted := strings.Fields(input)
	expected := []string{"BBAR", "150"}

	assert.Equal(t, reflect.DeepEqual(splitted, expected), true)
	assert.Equal(t, splitted[0], "BBAR")
	assert.Equal(t, splitted[0][0:1], "B")
}
