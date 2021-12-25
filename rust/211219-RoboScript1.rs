//https://www.codewars.com/kata/58708934a44cfccca60000c4

/* 
extern crate regex;
use regex::{Regex, Captures};

pub fn highlight(code: &str) -> String {
    let re = Regex::new(r"F+|L+|R+|\d+").unwrap();
    re.replace_all(code, |c: &Captures| match c[0].chars().next().unwrap() {
        'F' => format!(r#"<span style="color: pink">{}</span>"#, &c[0]),
        'L' => format!(r#"<span style="color: red">{}</span>"#, &c[0]),
        'R' => format!(r#"<span style="color: green">{}</span>"#, &c[0]),
        _ => format!(r#"<span style="color: orange">{}</span>"#, &c[0]),
    }).to_string()
}
*/

#[cfg(test)]
mod tests {
    #[test]
    fn test_stack() {
        let mut stack_number = vec![1, 2, 4, 3];        
        let input = 55;

        stack_number.push(input);

        assert_eq!(stack_number.pop().unwrap(), input);
    }

    #[test]
    fn test_string_contains() {
        let input = String::from("aaaa");

        assert_eq!(input.contains('a'), true);
    }

    fn to_type(input: char) -> i32 {
        if input == 'F' {
            return 0;
        } else if input == 'R' {
            return 1;
        } else if input == 'L' {
            return 2;
        }

        return 4;
    }

    fn to_tagged(input: &String) -> String {
        match to_type(input.chars().nth(0).unwrap()) {
            0 => format!("<span style=\"color: pink\">{}</span>", input),
            1 => format!("<span style=\"color: green\">{}</span>", input),
            2 => format!("<span style=\"color: red\">{}</span>", input),
            _ => format!("<span style=\"color: orange\">{}</span>", input),
        }
    }

    #[test]
    fn parse_statement() {
        let input: &str = "FFFR(FRL123)345F2LL";

        let mut output: Vec<String> = Vec::new();

        let mut last_type: i32 = -1;
        let mut letter_set = String::from("");
        
        input.chars().for_each(|letter: char| {
            if letter == '(' || letter == ')' {
                output.push(to_tagged(&letter_set));
                letter_set.clear();

                output.push(String::from(letter));
                return;
            }

            if !letter_set.is_empty() && to_type(letter) != last_type {
                output.push(to_tagged(&letter_set));

                letter_set.clear();
            }

            letter_set.push(letter);
            last_type = to_type(letter);
        });

        if !letter_set.is_empty() {
            output.push(to_tagged(&letter_set));
        }
    }

    fn highlight(code: &str) -> String {
        let mut output: Vec<String> = Vec::new();

        let mut last_type: i32 = -1;
        let mut letter_set = String::from("");

        code.chars().for_each(|letter: char| {
            if letter == '(' || letter == ')' {
                if !letter_set.is_empty() {
                    output.push(to_tagged(&letter_set));
                    letter_set.clear();
                }

                output.push(String::from(letter));
                return;
            }

            if !letter_set.is_empty() && to_type(letter) != last_type {
                output.push(to_tagged(&letter_set));

                letter_set.clear();
            }

            letter_set.push(letter);
            last_type = to_type(letter);
        });

        if !letter_set.is_empty() {
            output.push(to_tagged(&letter_set));
        }

        output.into_iter().collect()
    }

    macro_rules! assert_highlight {
        ($code:expr , $expected:expr $(,)*) => {{
            let actual = highlight($code);
            let expected = $expected;
            println!("Code without syntax highlighting: {}", $code);
            println!("Your code with syntax highlighting: {}", actual);
            println!("Expected syntax highlighting: {}", expected);
            assert_eq!(actual, expected);
        }};
    }

    #[test]
    fn examples_in_description() {
        assert_highlight!(
            "F3RF5LF7",
            r#"<span style="color: pink">F</span><span style="color: orange">3</span><span style="color: green">R</span><span style="color: pink">F</span><span style="color: orange">5</span><span style="color: red">L</span><span style="color: pink">F</span><span style="color: orange">7</span>"#,
        );
        assert_highlight!(
            "FFFR345F2LL",
            r#"<span style="color: pink">FFF</span><span style="color: green">R</span><span style="color: orange">345</span><span style="color: pink">F</span><span style="color: orange">2</span><span style="color: red">LL</span>"#,
        );
    }
}
