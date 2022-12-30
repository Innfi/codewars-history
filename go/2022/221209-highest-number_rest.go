// https://www.codewars.com/kata/5420fc9bb5b2c7fd57000004

package codewars_history

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func HighestRank(nums []int) int {
	numberMap := make(map[int]int)

	for _, elem := range nums {
		numberMap[elem] += 1
	}

	highest := 0
	highestCount := 0

	for key, val := range numberMap {
		if highestCount > val {
			continue
		}

		if highestCount < val || highest < key {
			highest = key
			highestCount = val
			continue
		}
	}

	return highest
}

func TestHighestRank(t *testing.T) {
	assert.Equal(t, HighestRank([]int{12, 10, 8, 12, 7, 6, 4, 10, 12}), 12)
	assert.Equal(t, HighestRank([]int{12, 10, 8, 12, 7, 6, 4, 10, 12, 10}), 12)
	assert.Equal(t, HighestRank([]int{12, 10, 8, 8, 3, 3, 3, 3, 2, 4, 10, 12, 10}), 3)
}
