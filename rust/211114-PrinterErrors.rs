//https://www.codewars.com/kata/56541980fa08ab47a0000040

/**
 * 
fn printer_error(s: &str) -> String {
    // Your cude here
    format!("{}/{}", s.chars().filter(|&c| c > 'm').count(), s.len())
}
 */

#[cfg(test)]
mod tests {
    #[test]
    fn test_comare_char() {
        assert_eq!('a' > 'b', false);
    }

    #[test]
    fn str_find_match_counts() {
        let input: &str = "abcde";

        let pos: usize = input.find(|x| {
            x > 'a'
        }).unwrap();

        assert_eq!(pos, 1);
    }

    fn printer_error(s: &str) -> String {
        let count: i32 = s.chars().map(|x| {
            match x > 'm' {
                true => 1, 
                _ => 0
            }
        }).sum::<i32>();
        
        String::from(format!("{}/{}", count, s.len()))
    }

    #[test]
    fn should_pass_all_the_tests_provided() {
        assert_eq!(&printer_error("aaaaaaaaaaaaaaaabbbbbbbbbbbbbbbbbbmmmmmmmmmmmmmmmmmmmxyz"), "3/56");
        assert_eq!(&printer_error("kkkwwwaaaaaaaaaaaaaabbbbbbbbbbbbbbbbbbmmmmmmmmmmmmmmmmmmmxyz"), "6/60");
        assert_eq!(&printer_error("kkkwwwaaaaaaaaaaaaaabbbbbbbbbbbbbbbbbbmmmmmmmmmmmmmmmmmmmxyzuuuuu"), "11/65");
    }
}
