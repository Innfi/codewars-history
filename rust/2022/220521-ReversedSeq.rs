//https://www.codewars.com/kata/5a00e05cc374cb34d100000d

#[cfg(test)]
mod tests {
    #[test]
    fn test_range() {
        let input: u32 = 5;

        let to_vec: Vec<u32> = (1..input+1).rev().collect();

        assert_eq!(to_vec, vec![5, 4, 3, 2, 1]);
    }

    fn reverse_seq(n: u32) -> Vec<u32> {
        (1..n+1).rev().collect()
    }

    #[test]
    fn sample_test() {
        assert_eq!(reverse_seq(5), [5, 4, 3, 2, 1].to_vec());
    }
}
