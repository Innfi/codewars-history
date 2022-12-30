// https://www.codewars.com/kata/5a3fe3dde1ce0e8ed6000097

package codewars_history

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func century(year int) int {
	prefix := year / 100
	start := (prefix) * 100

	if year > start {
		return prefix+1
	}

	return prefix
}

func TestIntDivision(t *testing.T) {
	remain := (1901 / 100) * 100

	assert.Equal(t, remain, 1900)
}

func TestBase(t *testing.T) {
	assert.Equal(t, 1, 1)
}

func TestCentury(t *testing.T) {
	assert.Equal(t, century(1901), 20)
	assert.Equal(t, century(1900), 19)
}