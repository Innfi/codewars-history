// https://www.codewars.com/kata/57d5e850bfcdc545870000b7

package codewars_history

import (
	"regexp"
	"sort"
	"strings"
	"testing"

	"github.com/stretchr/testify/assert"
)

func DeadAntCount(ants string) int {
	debris := strings.Replace(ants, "ant", " ", -1)

	array := []int{
		strings.Count(debris, "a"),
		strings.Count(debris, "n"),
		strings.Count(debris, "t"),
	}
	sort.Ints(array)

	return array[len(array)-1]
	// withoutAnts := strings.Replace(ants, "ant", " ", -1)
	// reg := regexp.MustCompile(`[a|n|t]+`)

	// debris := reg.FindAllString(withoutAnts, -1)
	// fmt.Println("debris: ", debris)
	// return len(debris)
}

func TestInit(t *testing.T) {
	input := "ant anantt aantnt"
	output := strings.Replace(input, "ant", " ", -1)
	assert.Equal(t, output, "  an t a nt")

	reg := regexp.MustCompile(`\s+`)
	afterRegex := reg.ReplaceAllString(output, " ")
	assert.Equal(t, afterRegex, " an t a nt")

	splitted := strings.Split(afterRegex, " ")

	assert.Equal(t, len(splitted), 5)
}

func TestCase(t *testing.T) {
	input := "t.nt.nt.a..ntan.a....aa.t..na...tantt.anantan..ta.nnt"

	assert.Equal(t, DeadAntCount(input), 9)
}
