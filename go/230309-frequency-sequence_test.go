// https://www.codewars.com/kata/585a033e3a36cdc50a00011c

package codewars_history

import (
	"strconv"
	"strings"
	"testing"

	"github.com/stretchr/testify/assert"
)

func FreqSeq(str string, sep string) string {
	countMap := make(map[rune]int)

	for _, elem := range str {
		countMap[elem] += 1
	}

	output := []string{}
	for _, elem := range str {
		temp := strconv.Itoa(countMap[elem])
		output = append(output, temp)
	}

	return strings.Join(output, sep)
}

func TestMap(t *testing.T) {
	input := make(map[string]int)

	input["a"] = 1
	input["b"] = 2
	input["a"] += 2

	assert.Equal(t, input["a"], 3)
	assert.Equal(t, input["c"], 0)
}

func TestFreqSeq(t *testing.T) {
	assert.Equal(t, FreqSeq("hello world", "-"), "1-1-3-3-2-1-1-2-1-3-1")
}
