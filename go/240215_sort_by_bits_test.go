// https://www.codewars.com/kata/59fa8e2646d8433ee200003f

package codewars_history

import (
	"fmt"
	"sort"
	"strings"
	"testing"

	"github.com/stretchr/testify/assert"
)

func SortByBit(arr []int) {
	sort.Slice(arr, func(i, j int) bool {
		binaryLeft := strings.Count(fmt.Sprintf("%b", arr[i]), "1")
		binaryRight := strings.Count(fmt.Sprintf("%b", arr[j]), "1")

		if binaryLeft < binaryRight {
			return true
		}
		if binaryLeft > binaryRight {
			return false
		}

		return arr[i] < arr[j]
	})
}

func TestSortByBit(t *testing.T) {
	input := []int{3, 8, 3, 6, 5, 7, 9, 1}
	SortByBit(input)
	assert.Equal(t, input, []int{1, 8, 3, 3, 5, 6, 9, 7})

	input2 := []int{9, 4, 5, 3, 5, 7, 2, 56, 8, 2, 6, 8, 0}
	SortByBit(input2)
	assert.Equal(t, input2, []int{0, 2, 2, 4, 8, 8, 3, 5, 5, 6, 9, 7, 56})
}

func TestSortSlice(t *testing.T) {
	input := []int{5, 3, 4, 1, 2, 6}
	sort.Slice(input, func(i, j int) bool {
		return input[i] > input[j]
	})

	assert.Equal(t, input, []int{6, 5, 4, 3, 2, 1})
}

type ExtInt []int

func (i ExtInt) Len() int {
	return len(i)
}

func (i ExtInt) Swap(a, b int) {
	i[a], i[b] = i[b], i[a]
}

func (i ExtInt) Less(a, b int) bool {
	return i[a] < i[b]
}

func TestSortByCustomType(t *testing.T) {
	input := []int{5, 3, 4, 1, 2, 6}

	sort.Sort(ExtInt(input))

	assert.Equal(t, input, []int{1, 2, 3, 4, 5, 6})
}

func TestCount(t *testing.T) {
	input := "10010011"

	assert.Equal(t, strings.Count(input, "1"), 4)
}

func TestBit(t *testing.T) {
	input := 7

	binary := fmt.Sprintf("%b", input)

	assert.Equal(t, binary, "111")
}
