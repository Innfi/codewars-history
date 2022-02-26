// https://www.codewars.com/kata/51e0007c1f9378fa810002a9

#[cfg(test)]
mod tests {
    #[test]
    fn str_split() {
        let input: &str = "abcde";
        let output: Vec<char> = vec!['a', 'b', 'c', 'd', 'e'];

        let splitted: Vec<char> = input.chars().collect();

        assert_eq!(splitted, output);
    }

    fn parse(code: &str) -> Vec<i32> {
        let mut output: Vec<i32> = Vec::new();
        let mut current = 0;

        code.chars().for_each(|x| {
            match x {
                'i' => current += 1,
                'd' => current -= 1,
                's' => current = current * current,
                'o' => output.push(current),
                _ => {},
            }
        });

        output
    }

    #[test]
    fn sample_tests() {
        assert_eq!(parse("iiisdoso"), vec![8, 64]);
        assert_eq!(parse("iiisdosodddddiso"), vec![8, 64, 3600]);
    }
}
