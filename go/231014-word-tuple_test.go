//https://www.codewars.com/kata/57a6633153ba33189e000074

package codewars_history

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

type Tuple struct {
	Char  rune
	Count int
}

func OrderedCount(text string) []Tuple {
	output := []Tuple{}
	indexMap := make(map[rune]int)
	currentIndex := 0

	for _, elem := range text {
		if indexMap[elem] <= 0 {
			output = append(output, Tuple{Char: elem, Count: 1})
			indexMap[elem] = currentIndex + 1
			currentIndex++

			continue
		}

		targetIndex := indexMap[elem] - 1
		output[targetIndex] = Tuple{
			Char:  elem,
			Count: output[targetIndex].Count + 1,
		}
	}

	return output
}

func TestInit(t *testing.T) {
	assert.Equal(t, 1, 1)
}

func TestStringRange(t *testing.T) {
	input := "abracadabra"

	for _, elem := range input {
		fmt.Println("elem: ", elem)
	}
}

func TestUpdateElement(t *testing.T) {
	input := []Tuple{
		{Char: 'a', Count: 1},
		{Char: 'c', Count: 10},
		{Char: 'B', Count: 78},
	}

	input[0] = Tuple{Char: 'a', Count: input[0].Count + 2}

	assert.Equal(t, input[0].Char, 'a')
}

func TestOrderedCount(t *testing.T) {
	input := "abracadabra"
	expected := []Tuple{
		{'a', 5},
		{'b', 2},
		{'r', 2},
		{'c', 1},
		{'d', 1},
	}

	output := OrderedCount(input)

	assert.Equal(t, expected, output)
}
