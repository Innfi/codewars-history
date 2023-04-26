// https://www.codewars.com/kata/57eaeb9578748ff92a000009

package codewars_history

import (
	"strconv"
	"testing"

	"github.com/stretchr/testify/assert"
)

func SumMixArr(arr []any) int {
	result := 0

	for _, elem := range arr {
		switch typedValue := elem.(type) {
		case int:
			result += typedValue
		case string:
			toInt, _ := strconv.Atoi(typedValue)
			result += toInt
		default:
			break
		}
	}

	return result
}

func TestSum(t *testing.T) {
	assert.Equal(t, SumMixArr([]any{9, 3, "7", "3"}), 22)
	assert.Equal(t, SumMixArr([]any{"5", "0", 9, 3, 2, 1, "9", 6, 7}), 42)
}
