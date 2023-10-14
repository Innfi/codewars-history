// https://www.codewars.com/kata/5899dc03bc95b1bf1b0000ad

package codewars_history

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func Invert(arr []int) []int {
	result := []int{}

	for _, elem := range arr {
		result = append(result, -elem)
	}

	return result
}

func TestInvert(t *testing.T) {
	assert.Equal(t, Invert([]int{1, 2, 3, 4, 5}), []int{-1, -2, -3, -4, -5})
}
