// https://www.codewars.com/kata/52597aa56021e91c93000cb0

package codewars_history

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func MoveZeros(arr []int) []int {
	result := []int{}
	zeroCount := 0

	for _, elem := range arr {
		if elem == 0 {
			zeroCount += 1
			continue
		}

		result = append(result, elem)
	}

	for i := 0; i < zeroCount; i++ {
		result = append(result, 0)
	}

	return result
}

func TestMoveZeros(t *testing.T) {
	assert.Equal(t,
		MoveZeros([]int{1, 2, 0, 1, 0, 1, 0, 3, 0, 1}),
		[]int{1, 2, 1, 1, 3, 1, 0, 0, 0, 0})
}
