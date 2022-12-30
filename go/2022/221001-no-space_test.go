// https://www.codewars.com/kata/57eae20f5500ad98e50002c5

package codewars_history

import (
	"testing"

	"github.com/stretchr/testify/assert"

	"strings"
)

func NoSpace(word string) string {
	occupied := strings.ReplaceAll(word, " ", "")

	return occupied
}

func TestCase(t *testing.T) {
	assert.Equal(t, NoSpace("8 j 8   mBliB8g  imjB8B8  jl  B"), "8j8mBliB8gimjB8B8jlB")
}
