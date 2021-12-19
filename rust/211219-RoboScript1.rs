//https://www.codewars.com/kata/58708934a44cfccca60000c4

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

    #[test]
    fn parse_statement() {
        let input: &str = "FFFR345F2LL";

        let mut letter_set = String::from("");
        let mut letters: Vec<String> = Vec::new();

        input.chars().for_each(|letter: char| {
            if letter_set.len() > 0 && letter_set.contains(letter) {
                letters.push(letter_set);
                letter_set.clear();
            } else {
                //letter_set.push(letter);
            }
        });
    }

    // fn highlight(code: &str) -> String {
        
    // }

    // macro_rules! assert_highlight {
    //     ($code:expr , $expected:expr $(,)*) => {{
    //         let actual = highlight($code);
    //         let expected = $expected;
    //         println!("Code without syntax highlighting: {}", $code);
    //         println!("Your code with syntax highlighting: {}", actual);
    //         println!("Expected syntax highlighting: {}", expected);
    //         assert_eq!(actual, expected);
    //     }};
    // }

    // #[test]
    // fn examples_in_description() {
    //     assert_highlight!(
    //         "F3RF5LF7",
    //         r#"<span style="color: pink">F</span><span style="color: orange">3</span><span style="color: green">R</span><span style="color: pink">F</span><span style="color: orange">5</span><span style="color: red">L</span><span style="color: pink">F</span><span style="color: orange">7</span>"#,
    //     );
    //     assert_highlight!(
    //         "FFFR345F2LL",
    //         r#"<span style="color: pink">FFF</span><span style="color: green">R</span><span style="color: orange">345</span><span style="color: pink">F</span><span style="color: orange">2</span><span style="color: red">LL</span>"#,
    //     );
    // }
}
