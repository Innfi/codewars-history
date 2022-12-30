// https://www.codewars.com/kata/536a155256eb459b8700077e

package codewars_history

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func MakeSpirial(n int) [][]int {
	spiral := make([][]int, n)
	for i := range spiral {
		spiral[i] = make([]int, n)
	}

	return spiral
}

func MarkRight(spiral *[][]int, row *int, col *int, current *int) {
	size := len(*spiral)
	if *current >= size*size {
		return
	}

	for *row < size {
		if (*spiral)[*col][*row] > 0 {
			*row += 1
			continue
		}

		*current += 1
		(*spiral)[*col][*row] = *current
		if *row+1 >= size || (*spiral)[*col][*row+1] > 0 {
			return
		}

		*row += 1
	}
}

func MarkDown(spiral *[][]int, row *int, col *int, current *int) {
	size := len(*spiral)
	if *current >= size*size {
		return
	}

	for *col < size {
		if (*spiral)[*col][*row] > 0 {
			*col += 1
			continue
		}

		*current += 1
		(*spiral)[*col][*row] = *current
		if *col+1 >= size || (*spiral)[*col+1][*row] > 0 {
			return
		}

		*col += 1
	}
}

func MarkLeft(spiral *[][]int, row *int, col *int, current *int) {
	size := len(*spiral)
	if *current >= size*size {
		return
	}

	for *row >= 0 {
		if (*spiral)[*col][*row] > 0 {
			*row -= 1
			continue
		}

		*current += 1
		(*spiral)[*col][*row] = *current
		if *row-1 < 0 || (*spiral)[*col][*row-1] > 0 {
			return
		}

		*row -= 1
	}
}

func MarkUp(spiral *[][]int, row *int, col *int, current *int) {
	size := len(*spiral)
	if *current >= size*size {
		return
	}

	for *col >= 0 {
		if (*spiral)[*col][*row] > 0 {
			*col -= 1
			continue
		}

		*current += 1
		(*spiral)[*col][*row] = *current
		if *col-1 < 0 || (*spiral)[*col-1][*row] > 0 {
			return
		}

		*col -= 1
	}
}

func CreateSpiral(n int) [][]int {
	if n <= 0 {
		return [][]int{}
	}

	if n == 1 {
		return [][]int{{1}}
	}

	spiral := MakeSpirial(n)

	count := n * n
	row := 0
	col := 0
	current := 0

	for current < count {
		MarkRight(&spiral, &row, &col, &current)
		MarkDown(&spiral, &row, &col, &current)
		MarkLeft(&spiral, &row, &col, &current)
		MarkUp(&spiral, &row, &col, &current)
	}

	return spiral
}

func TestSpiralSize(t *testing.T) {
	spiral := MakeSpirial(3)

	assert.Equal(t, len(spiral), 3)

	for i := range spiral {
		assert.Equal(t, len(spiral[i]), 3)
	}
}

func TestMarkRight(t *testing.T) {
	spiral := MakeSpirial(5)

	row := 0
	col := 0
	current := 0

	MarkRight(&spiral, &row, &col, &current)

	assert.Equal(t, row, 4)
	assert.Equal(t, col, 0)
	assert.Equal(t, current, 5)

	assert.Equal(t, spiral[0][0], 1)
	assert.Equal(t, spiral[0][1], 2)
	assert.Equal(t, spiral[0][2], 3)
	assert.Equal(t, spiral[0][3], 4)
	assert.Equal(t, spiral[0][4], 5)
}

func TestRightAndDown(t *testing.T) {
	spiral := MakeSpirial(5)

	row := 0
	col := 0
	current := 0

	MarkRight(&spiral, &row, &col, &current)
	MarkDown(&spiral, &row, &col, &current)

	assert.Equal(t, current, 9)

	assert.Equal(t, spiral[0][4], 5)
	assert.Equal(t, spiral[1][4], 6)
	assert.Equal(t, spiral[2][4], 7)
	assert.Equal(t, spiral[3][4], 8)
	assert.Equal(t, spiral[4][4], 9)
}

func TestSpiral(t *testing.T) {
	assert.Equal(t, CreateSpiral(1), [][]int{{1}})
	assert.Equal(t, CreateSpiral(2), [][]int{{1, 2}, {4, 3}})
	assert.Equal(t, CreateSpiral(3), [][]int{{1, 2, 3}, {8, 9, 4}, {7, 6, 5}})
	assert.Equal(t, CreateSpiral(4), [][]int{
		{1, 2, 3, 4},
		{12, 13, 14, 5},
		{11, 16, 15, 6},
		{10, 9, 8, 7},
	})
	assert.Equal(t, CreateSpiral(5), [][]int{
		{1, 2, 3, 4, 5},
		{16, 17, 18, 19, 6},
		{15, 24, 25, 20, 7},
		{14, 23, 22, 21, 8},
		{13, 12, 11, 10, 9},
	})
}
