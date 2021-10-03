//https://www.codewars.com/kata/5502c9e7b3216ec63c0001aa

/**
 * fn open_or_senior(data: Vec<(i32, i32)>) -> Vec<String> {
    data.into_iter()
        .map(|(age, handicap)| {
            if age >= 55 && handicap > 7 {
                "Senior"
            } else {
                "Open"
            }
            .to_string()
        })
        .collect()
}
 */

#[cfg(test)]
mod tests {
    fn open_or_senior(data: Vec<(i32, i32)>) -> Vec<String> {
        data.iter().map(|(a, b)| {
            if *a >= 55 && *b > 7 {
                return String::from("Senior");
            } else {
                return String::from("Open");
            }
        }).collect()   
    }

    #[test]
    fn figure_paren() {
        let test_set = (45, 12);
        assert_eq!(test_set.0, 45);
        assert_eq!(test_set.1, 12); 
    }

    #[test]
    fn try_compare() {
        let test_set = (45, 12);

        let result: bool = test_set.0 > 55 && test_set.1 > 7;
        assert_eq!(result, false);

        let test_set2 = (55, 21);
        let result2: bool = test_set2.0 >= 55 && test_set2.1 > 7;
        assert_eq!(result2, true); 
    }

    #[test]
    fn map_vector() {
        let input: Vec<(i32, i32)> = vec![(45, 12), (55,21), (19, -2), (104, 20)];
        let first_elem: (i32, i32) = *input.first().unwrap();

        let map_result: Vec<i32> = input.iter().map(|(a, b)| a+b).collect();
        assert_eq!(*map_result.first().unwrap(), first_elem.0 + first_elem.1);
    }

    #[test]
    fn to_member_string() {
        let input: Vec<(i32, i32)> = vec![(45, 12), (55,21), (19, -2), (104, 20)];
        let map_result: Vec<String> = input.iter()
            .map(|(a, b)| {
                if *a >= 55 && *b > 7 {
                    return String::from("Senior");
                } else {
                    return String::from("Open");
                }
            }).collect();
        
        assert_eq!(map_result, vec!["Open", "Senior", "Open", "Senior"]);
    }

    #[test]
    fn returns_expected() {
        assert_eq!(open_or_senior(vec![(45, 12), (55,21), (19, -2), (104, 20)]), vec!["Open", "Senior", "Open", "Senior"]);
        assert_eq!(open_or_senior(vec![(3, 12), (55,1), (91, -2), (54, 23)]), vec!["Open", "Open", "Open", "Open"]);
    }
}
