// https://www.codewars.com/kata/59706036f6e5d1e22d000016
package codewars_history

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func WordsToMarks(s string) int {
	result := 0
	for _, elem := range s {
		result += int(elem) - 96
	}

	return result
}

func TestCharToNumber(t *testing.T) {
	input := 'a'
	asNumber := int(input)

	assert.Equal(t, asNumber, 97)
}

func TestConv(t *testing.T) {
	assert.Equal(t, WordsToMarks("attitude"), 100)
	assert.Equal(t, WordsToMarks("selfness"), 99)
	assert.Equal(t, WordsToMarks("knowledge"), 96)
}
