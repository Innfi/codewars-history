//https://www.codewars.com/kata/56bc28ad5bdaeb48760009b0

package codewars_history

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestStringSlice(t *testing.T) {
	assert.Equal(t, "abc"[1:], "bc")
	assert.Equal(t, "abc"[:1], "a")
	assert.Equal(t, "abcde"[1:4], "bcd")
	assert.Equal(t, "abcde"[1:len("abcde")-1], "bcd")
}

func RemoveChar(word string) string {
	return word[1 : len(word)-1]
}

func TestRemoveChar(t *testing.T) {
	assert.Equal(t, RemoveChar("abcd"), "bc")
}
