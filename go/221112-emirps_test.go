// https://www.codewars.com/kata/55de8eabd9bef5205e0000ba

package codewars_history

import (
	"math/big"
	"testing"

	"github.com/fxtlabs/primes"

	"github.com/stretchr/testify/assert"
)

func reverseNumber(input int) int {
	reversed := 0
	for input > 0 {
		remainder := input % 10
		reversed = (reversed * 10) + remainder
		input /= 10
	}

	return reversed
}

func TestProbablyPrime(t *testing.T) {
	assert.Equal(t, big.NewInt(1234).ProbablyPrime(0), false)
	assert.Equal(t, big.NewInt(79).ProbablyPrime(0), true)
}

func TestReverseNumber(t *testing.T) {
	assert.Equal(t, reverseNumber(12), 21)
	assert.Equal(t, reverseNumber(12346), 64321)
}

func TestPrimeGenerator(t *testing.T) {
	p := primes.Sieve(10)

	assert.Equal(t, len(p), 4)
	assert.Equal(t, p, []int{2, 3, 5, 7})
}

func TestMap(t *testing.T) {
	testMap := make(map[int]bool)

	testMap[1] = true

	assert.Equal(t, testMap[1], true)
	assert.Equal(t, testMap[3], false)
}

func FindEmirp(n int) [3]int {
	primes := primes.Sieve(n)

	primeMap := make(map[int]bool)
	for _, prime := range primes {
		primeMap[prime] = true
	}

	sum := 0
	largest := 0
	len := 0
	for _, key := range primes {
		if key <= 11 {
			continue
		}

		reversed := reverseNumber(key)
		if big.NewInt(int64(reversed)).ProbablyPrime(0) == false {
			continue
		}

		sum += key
		len += 1
		if largest < key {
			largest = key
		}
	}

	return [3]int{len, largest, sum}
}

func TestFindEmirp(t *testing.T) {
	assert.Equal(t, FindEmirp(50), [3]int{4, 37, 98})
}
