// https://www.codewars.com/kata/5842df8ccbd22792a4000245

#[cfg(test)]
mod tests {
    #[test]
    fn test_spread_number() {
        let input: u64 = 1234;
        let expected_output: Vec<char> = vec!['1', '2', '3', '4'];

        let output: Vec<char> = format!("{}", input).chars().collect();

        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_pow() {
        let index: usize = 4;

        assert_eq!(10u64.pow(index as u32), 10000);
    }

    fn expanded_form(n: u64) -> String {
        let spread: Vec<char> = format!("{}", n).chars().collect();
        
        spread
        .iter()
        .enumerate()
        .filter(|x| x.1 != &'0')
        .map(|(index, x)| {
            format!("{}",
                x.to_digit(10).unwrap() as u64 *
                10u64.pow((spread.len() - index-1) as u32)
            )
        }).collect::<Vec<String>>().join(" + ")
    }

    #[test]
    fn examples() {
        assert_eq!(expanded_form(12), "10 + 2");
        assert_eq!(expanded_form(42), "40 + 2");
        assert_eq!(expanded_form(70304), "70000 + 300 + 4");
    }
}
