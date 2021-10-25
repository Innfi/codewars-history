//https://www.codewars.com/kata/5656b6906de340bd1b0000ac

/**
 * 
use std::collections::BTreeSet;
fn longest(a1: &str, a2: &str) -> String {
    a1.chars()
        .chain(a2.chars())
        .collect::<BTreeSet<char>>()
        .iter()
        .collect()
}
 */

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    #[test]
    fn sort_chars() {
        let input: &str = "aretheyhere";
        let mut splitted: Vec<char> = input.chars().collect();
        splitted.sort_by(|a, b| a.cmp(b));

        let result: String = splitted.into_iter().collect();

        assert_eq!(result, String::from("aeeeehhrrty"));
    }

    #[test]
    fn test_hashset() {
        let input: &str = "aaabbccc";
        let to_set: BTreeSet<char> = input.chars().collect();
        let output: String = to_set.into_iter().collect();

        assert_eq!(output, String::from("abc"));
    }

    fn longest(a1: &str, a2: &str) -> String {
        let mut input: String = String::from(a1);
        input.push_str(a2);

        let to_set: BTreeSet<char> = input.chars().collect();

        to_set.into_iter().collect()
    }

    fn testing(s1: &str, s2: &str, exp: &str) -> () {
        println!("s1:{:?} s2:{:?}", s1, s2);
        println!("{:?} {:?}", longest(s1, s2), exp);
        println!("{}", longest(s1, s2) == exp);
        assert_eq!(&longest(s1, s2), exp)
    }

    #[test]
    fn basic_tests() {
        testing("aretheyhere", "yestheyarehere", "aehrsty");
        testing("loopingisfunbutdangerous", "lessdangerousthancoding", "abcdefghilnoprstu");
    }
}
