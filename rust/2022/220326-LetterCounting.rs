//https://www.codewars.com/kata/59e19a747905df23cb000024

/*
use std::collections::HashMap;

fn string_letter_count(s: &str) -> String {
    let mut items: Vec<(char, usize)> = {
        let mut counter = HashMap::new();
        for l in s.to_lowercase().chars().filter(|c| c.is_alphabetic()) {
            let c = counter.entry(l).or_default();
            *c += 1;
        }
        counter.drain().collect()
    };
    items.sort_unstable();
    items.iter().map(|(l, c)| format!("{}{}", c, l)).collect()
}
*/

#[cfg(test)]
mod tests {
    #[test]
    fn string_lower() {
        let lower = "The".to_ascii_lowercase();

        assert_eq!(lower, String::from("the"));
    }

    #[test]
    fn string_retain() {
        let mut input = String::from("a b c ");
        input.retain(|x| !x.is_whitespace());

        assert_eq!(input.as_str(), "abc");
    }

    #[test]
    fn string_sorted() {
        const input: &str = &"The quick brown fox jumps over the lazy dog.";
        let mut input_lower = input.to_ascii_lowercase();
        input_lower.retain(|x| !x.is_whitespace() && x.is_alphabetic());

        let mut output_vec: Vec<char> = input_lower.chars().collect();
        output_vec.sort();

        let output2: String = output_vec.into_iter().collect();

        assert_eq!(output2, String::from("abcdeeefghhijklmnoooopqrrsttuuvwxyz"));
    }

    fn string_letter_count(s: &str) -> String {
        let mut input_lower = s.to_ascii_lowercase();
        input_lower.retain(|x| !x.is_whitespace() && x.is_alphabetic());

        if input_lower.is_empty() { return String::from(""); }

        let mut output_vec: Vec<char> = input_lower.chars().collect();
        output_vec.sort();

        let mut current_letter: char = output_vec[0];
        let mut count = 0;
        let mut output_str = String::new();

        output_vec.into_iter().for_each(|x: char| {
            if current_letter == x {
                count += 1;
            } else {
                output_str.push_str(format!("{}{}", count, current_letter).as_str());

                current_letter = x;
                count = 1;
            }
        });
        output_str.push_str(format!("{}{}", count, current_letter).as_str());

        output_str
    }

    #[test]
    fn test_basic() {
        assert_eq!(string_letter_count(&"The quick brown fox jumps over the lazy dog."), "1a1b1c1d3e1f1g2h1i1j1k1l1m1n4o1p1q2r1s2t2u1v1w1x1y1z");
        assert_eq!(string_letter_count(&"The time you enjoy wasting is not wasted time."), "2a1d5e1g1h4i1j2m3n3o3s6t1u2w2y");
        assert_eq!(string_letter_count(&"./4592#{}()"), "");
    }
}
