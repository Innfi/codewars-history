//https://www.codewars.com/kata/5174a4c0f2769dd8b1000003

#[cfg(test)]
mod tests {
    fn sort_numbers(arr: &Vec<i32>) -> Vec<i32> {
        let mut out = arr.to_vec();
        out.sort();

        out
    }

    #[test]
    fn sample_tests() {
        assert_eq!(sort_numbers(&vec![1, 2, 3, 10, 5]), vec![1, 2, 3, 5, 10]);
        assert_eq!(sort_numbers(&vec![1, 2, 3, 10, 5]), vec![1, 2, 3, 5, 10]);
        assert_eq!(sort_numbers(&vec![]), vec![]);
        assert_eq!(sort_numbers(&vec![20, 2, 10]), vec![2, 10, 20]);
        assert_eq!(sort_numbers(&vec![2, 20, 10]), vec![2, 10, 20]);
        assert_eq!(sort_numbers(&vec![2, 10, 20]), vec![2, 10, 20]);
    }
}
