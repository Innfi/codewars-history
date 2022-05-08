//https://www.codewars.com/kata/58f5c63f1e26ecda7e000029

/* 
fn wave(s: &str) -> Vec<String> {
    s.char_indices()
        .map(|(i, c)| s[..i].to_string() + &c.to_uppercase().to_string() + &s[i + 1..])
        .filter(|wave| wave != s)
        .collect()
}
*/

#[cfg(test)]
mod tests {
    #[test]
    fn str_indexing() {
        let input = "hello";

        assert_eq!(input.chars().nth(0).unwrap(), 'h');
    }

    #[test]
    fn str_enumerate() {
        let input = "abcd";

        let output: Vec<String> = input.chars().enumerate().map(|(index, value)| {
            let mut out = String::from(input);
            let uppercase = format!("{}", value);

            out.replace_range(index..index+1, uppercase.to_uppercase().as_str());

            out
        }).collect();

        let expected = ["Abcd", "aBcd", "abCd", "abcD"];

        assert_eq!(output, expected);
    }

    fn wave(s: &str) -> Vec<String> {
        let mut wave_out: Vec<String> = Vec::new();

        s.chars().enumerate().for_each(|(index, value)| {
            if value == ' ' { return; }

            let mut splitted: Vec<char> = s.chars().collect();
            splitted[index] = value.to_uppercase().nth(0).unwrap();
            wave_out.push(splitted.into_iter().collect());
        });

        wave_out
    }

    #[test]
    fn test_examples() {
        let expect = ["Hello", "hEllo", "heLlo", "helLo", "hellO"];
        assert_eq!(wave("hello"), expect);
        
        let expect = ["Codewars", "cOdewars", "coDewars", "codEwars", "codeWars", "codewArs", "codewaRs", "codewarS"];
        assert_eq!(wave("codewars"), expect);
        
        let expect: [&str; 0] = [];
        assert_eq!(wave(""), expect);
        
        let expect = ["Two words", "tWo words", "twO words", "two Words", "two wOrds", "two woRds", "two worDs", "two wordS"];
        assert_eq!(wave("two words"), expect);
        
        let expect = [" Gap ", " gAp ", " gaP "];
        assert_eq!(wave(" gap "), expect);
    }
}
