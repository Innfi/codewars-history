// https://www.codewars.com/kata/526dbd6c8c0eb53254000110

package codewars_history

import (
	"testing"

	"github.com/stretchr/testify/assert"

	"regexp"
)

func alphanumeric(str string) bool {
	compileResult := regexp.MustCompile(`^\w+$`)

	return compileResult.MatchString(str)
}

func TestAlphanumeric(t *testing.T) {
	assert.Equal(t, alphanumeric(".*?"), false)
	assert.Equal(t, alphanumeric("hello world_"), false)
	assert.Equal(t, alphanumeric("     "), false)
	assert.Equal(t, alphanumeric("43534h56jmTHHF3k"), true)
}
