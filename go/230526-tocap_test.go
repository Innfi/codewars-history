// https://www.codewars.com/kata/59cfc000aeb2844d16000075

/*
func Capitalize(s string) []string {
  a, b := []rune(s),[]rune(s)
  for i := range a {
    if i%2 == 0 {
      a[i] = unicode.ToUpper(a[i])
    }else{
      b[i] = unicode.ToUpper(b[i])
    }
  }
  return []string{string(a), string(b)}
}
*/

package codewars_history

import (
	"strings"
	"testing"
	"unicode"

	"github.com/stretchr/testify/assert"
)

func Capitalize(st string) []string {
	var lhs strings.Builder
	var rhs strings.Builder
	for index, elem := range st {
		if index%2 == 0 {
			lhs.WriteRune(unicode.ToUpper(elem))
			rhs.WriteRune(unicode.ToLower(elem))
			continue
		}

		lhs.WriteRune(unicode.ToLower(elem))
		rhs.WriteRune(unicode.ToUpper(elem))
	}

	return []string{lhs.String(), rhs.String()}
}

func TestCapitalize(t *testing.T) {
	assert.Equal(t, Capitalize("aab"), []string{"AaB", "aAb"})
}
