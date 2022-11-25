// https://www.codewars.com/kata/545a4c5a61aa4c6916000755

package codewars_history

import (
	"testing"

	"sort"

	"github.com/stretchr/testify/assert"
)

func Gimme(array [3]int) int {
	indexMap := map[int]int{}

	for index, elem := range array {
		indexMap[elem] = index
	}

	sort.Ints(array[:])

	return indexMap[array[1]]
}

func TestFindMiddleElement(t *testing.T) {
	assert.Equal(t, Gimme([3]int{2, 3, 1}), 0)
}
