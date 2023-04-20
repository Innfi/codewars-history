// https://www.codewars.com/kata/554e4a2f232cdd87d9000038

/*
package kata

import "strings"

var dnaReplacer *strings.Replacer = strings.NewReplacer(
  "A", "T",
  "T", "A",
  "C", "G",
  "G", "C",
)

func DNAStrand(dna string) string {
  return dnaReplacer.Replace(dna)
}
*/

package codewars_history

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func DNAStrand(dna string) string {
	// your code here
	pairs := map[rune]string{
		'A': "T",
		'T': "A",
		'C': "G",
		'G': "C",
	}

	result := ""
	for _, elem := range dna {
		result += pairs[elem]
	}

	return result
}

func TestDNAStrand(t *testing.T) {
	assert.Equal(t, DNAStrand("AAAA"), "TTTT")
}
