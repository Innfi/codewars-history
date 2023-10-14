// https://www.codewars.com/kata/57f8ee485cae443c4d000127

package codewars_history

import (
	"strings"
	"testing"

	"github.com/stretchr/testify/assert"
)

func Spacify(s string) string {
	return strings.Join(strings.Split(s, ""), " ")
}

func TestSpacify(t *testing.T) {
	assert.Equal(t, Spacify("abc"), "a b c")
}
