// https://www.codewars.com/kata/57a04da9e298a7ee43000111
package codewars_history

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func ReverseList(lst []int) []int {
	resultArray := []int{}
	for i := len(lst) - 1; i >= 0; i-- {

		resultArray = append(resultArray, lst[i])
	}

	return resultArray
}

func TestReverseSort(t *testing.T) {
	assert.Equal(t, ReverseList([]int{1, 2, 3}), []int{3, 2, 1})
	assert.Equal(t, ReverseList([]int{-1, 1, 2, 3}), []int{3, 2, 1, -1})
}
