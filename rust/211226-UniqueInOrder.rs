//https://www.codewars.com/kata/54e6533c92449cc251001667

#[cfg(test)]
mod tests {
    #[test]
    fn test_regex() {
        let mut input = vec![1, 2, 2, 2, 3, 3, 4, 4, 4, 1, 1, 2];
        input.dedup();

        assert_eq!(input, vec![1, 2, 3, 4, 1, 2]);
    }

    fn unique_in_order<T>(
        sequence: T
    ) -> Vec<T::Item> where
        T: std::iter::IntoIterator,
        T::Item: std::cmp::PartialEq + std::fmt::Debug,
    {
        let mut test: Vec<T::Item> = sequence.into_iter().collect();
        test.dedup();

        test
    }

    #[test]
    fn sample_test() {
        assert_eq!(
            unique_in_order("AAAABBBCCDAABBB".chars()), 
            vec!['A','B','C','D','A','B']
        );
    }
}
