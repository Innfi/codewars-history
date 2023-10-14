// https://www.codewars.com/kata/5a6663e9fd56cb5ab800008b
package codewars_history

func CatYears(years int) int {
	if years == 1 {
		return 15
	} else if years == 2 {
		return 15 + 9
	} else {
		return (years-2)*4 + 15 + 9
	}
}

func DogYears(years int) int {
	if years == 1 {
		return 15
	} else if years == 2 {
		return 15 + 9
	} else {
		return (years-2)*5 + 15 + 9
	}
}

func CalculateYears(years int) (result [3]int) {
	// Write your solution here
	return [3]int{years, CatYears(years), DogYears(years)}
}
