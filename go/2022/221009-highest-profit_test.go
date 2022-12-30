// https://www.codewars.com/kata/559590633066759614000063

package codewars_history

import (
	"testing"

	"github.com/stretchr/testify/assert"

	"sort"
)

func MinMax(arr []int) [2]int {
	sort.Slice(arr, func(i, j int) bool {
		return arr[i] < arr[j]
	})

	return [2]int{arr[0], arr[len(arr)-1]}
}

func TestSortArray(t *testing.T) {
	input := []int{5, 4, 1, 2, 3, 10}

	sort.Slice(input, func(i, j int) bool {
		return input[i] < input[j]
	})

	assert.Equal(t, input, []int{1, 2, 3, 4, 5, 10})
}

func TestMinMax(t *testing.T) {
	assert.Equal(t, MinMax([]int{1, 2, 3, 4, 5}), [2]int{1, 5})
	assert.Equal(t, MinMax([]int{2334454, 5}), [2]int{5, 2334454})
}
