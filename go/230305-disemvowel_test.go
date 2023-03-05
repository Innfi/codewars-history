// https://www.codewars.com/kata/52fba66badcd10859f00097e

package codewars_history

import (
	"regexp"
	"testing"

	"github.com/stretchr/testify/assert"
)

func Disemvowel(comment string) string {
	r, _ := regexp.Compile("[aeiouAEIOU]")

	return r.ReplaceAllString(comment, "")
}

func TestDisemvowel(t *testing.T) {
	assert.Equal(t, Disemvowel("this"), "ths")
	assert.Equal(t, Disemvowel("This website is for losers LOL!"), "Ths wbst s fr lsrs LL!")
}
