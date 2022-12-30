// https://www.codewars.com/kata/55de8eabd9bef5205e0000ba

package codewars_history

import (
	"math/big"
	"testing"

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

func TestReverseNumber(t *testing.T) {
	assert.Equal(t, reverseNumber(12), 21)
	assert.Equal(t, reverseNumber(12346), 64321)
}

func GeneratePrimes(n int) []int {
	if n <= 2 {
		return []int{2}
	}

	primes := []int{2}
	current := 3
	for current <= n {
		if big.NewInt(int64(current)).ProbablyPrime(0) == true {
			primes = append(primes, current)
		}

		current += 2
	}

	return primes
}

func TestGeneratePrimes(t *testing.T) {
	primes := GeneratePrimes(11)
	assert.Equal(t, len(primes), 5)
	assert.Equal(t, primes, []int{2, 3, 5, 7, 11})
}

func GeneratePrimeMap(n int) map[int]int {
	primeMap := make(map[int]int)
	if n <= 2 {
		primeMap[2] = 2
		return primeMap
	}

	primeMap[2] = 2
	primeMap[3] = 3
	current := 3
	for current <= n {
		if big.NewInt(int64(current)).ProbablyPrime(0) == false {
			current += 2
			continue
		}

		reversed := reverseNumber(current)
		if reversed%2 == 0 {
			current += 2
			continue
		}

		if big.NewInt(int64(reversed)).ProbablyPrime(0) == true {
			primeMap[current] = reversed
		}

		current += 2
	}

	return primeMap
}

func TestPrimeMap(t *testing.T) {
	primeMap := GeneratePrimeMap(13)

	assert.Equal(t, len(primeMap), 6)

	assert.Equal(t, primeMap[12], 0)
	assert.Equal(t, primeMap[13], 31)
}

func TestProbablyPrime(t *testing.T) {
	assert.Equal(t, big.NewInt(1234).ProbablyPrime(0), false)
	assert.Equal(t, big.NewInt(79).ProbablyPrime(0), true)
}

func TestMap(t *testing.T) {
	testMap := make(map[int]bool)

	testMap[1] = true

	assert.Equal(t, testMap[1], true)
	assert.Equal(t, testMap[3], false)
}

func FindEmirp(n int) [3]int {
	primeMap := GeneratePrimeMap(n)
	sum := 0
	largest := 0
	len := 0

	for key, value := range primeMap {
		if key == value || value == 0 {
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

func FindEmirp3(n int) [3]int {
	primes := GeneratePrimes(n)

	primeMap := make(map[int]bool)
	for _, prime := range primes {
		primeMap[prime] = true
	}

	sum := 0
	largest := 0
	len := 0
	for _, key := range primes {
		reversed := reverseNumber(key)
		if key == reversed {
			continue
		}
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
	assert.Equal(t, FindEmirp(200), [3]int{15, 199, 1489})
}
