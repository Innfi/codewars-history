//https://www.codewars.com/kata/5208f99aee097e6552000148

/*
best practices: 

fn solution(s: &str) -> String {
    let mut res = String::new();
    for c in s.chars() {
        if c.is_uppercase() {
            res.push(' ');
        }
        res.push(c);
    }
    res
}
*/

#[cfg(test)]
mod tests {
    use substring::Substring;

    fn solution(s: &str) -> String {
        let matches: Vec<(usize, &str)> = 
            s.match_indices(|c: char| c.is_ascii_uppercase()).collect();
        let mut output: String = String::from("");
        let mut index_start = 0;

        matches.into_iter().for_each(|(index, _token)| {
            output.push_str(" ");
            output.push_str(  &s[index_start..index]  );
            index_start = index;
        });

        output.push_str(" ");
        output.push_str(&s[index_start..s.len()]);

        String::from(output.trim_start())
    }

    #[test]
    fn test_vector() {
        let input: &str = "camelCaseWordsHere";
        let input_string: String = String::from(input);
        let result: Vec<(usize, &str)> = 
            input_string.match_indices(|c: char| c.is_ascii_uppercase()).collect();

        let mut output: String = String::from("");
        let mut start = 0;
        
        let iters = result.into_iter();
        iters.for_each(|(index, _token)| {
            output.push_str(" ");
            output.push_str(input_string.substring(start, index));
            start = index;  
        });

        output.push_str(" ");
        output.push_str(input_string.substring(start, input_string.len()));
        let output_string: String = String::from(output.trim_start());

        assert_eq!(output_string, String::from("camel Case Words Here"));
    }

    #[test]
    fn test_solution() {
        assert_eq!(solution("camelCasing"), "camel Casing");
        assert_eq!(solution("camelCasingTest"), "camel Casing Test");
    }
}
