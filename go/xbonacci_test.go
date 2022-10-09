// https://www.codewars.com/kata/556e0fccc392c527f20000c5

package codewars_history

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func sum(array []int) int {
	var result int = 0
	for _, v := range array {
		result += v
	}

	return result
}

func Xbonacci(signature []int, n int) []int {
	result := signature
	arrayLen := len(result)

	if arrayLen >= n {
		return result[0:n]
	}

	for i := 0; i < n-arrayLen; i++ {
		result = append(result, sum(result[i:i+arrayLen]))
	}

	return result
}

func TestXbonacci(t *testing.T) {
	assert.Equal(t,
		Xbonacci([]int{0, 1}, 10),
		[]int{0, 1, 1, 2, 3, 5, 8, 13, 21, 34},
	)
	assert.Equal(t,
		Xbonacci([]int{1, 1}, 10),
		[]int{1, 1, 2, 3, 5, 8, 13, 21, 34, 55},
	)
	assert.Equal(t,
		Xbonacci([]int{1, 2, 3, 4, 5, 6, 7, 8, 9, 0}, 9),
		[]int{1, 2, 3, 4, 5, 6, 7, 8, 9},
	)
}

func TestArrayLength(t *testing.T) {
	input := []int{0, 0, 0, 0, 1}

	assert.Equal(t, len(input), 5)
}

func TestArraySlice(t *testing.T) {
	input := []int{1, 2, 3, 4, 5}
	start := 1
	end := 4

	assert.Equal(t, input[start:end], []int{2, 3, 4})
}
