//https://www.codewars.com/kata/5208f99aee097e6552000148

#[cfg(test)]
mod tests {
    use regex::Regex;
    use std::collections::HashSet;

    // fn solution(s: &str) -> String {
    //     String::from("test")
    // }

    #[test]
    fn find_uppercase() {
        let input: &str = "camelCaseInput";

        let splitted: Vec<&str> = 
            Regex::new(r"\p{Lu}").unwrap().split(input).collect();
        splitted.into_iter().for_each(|token| {
            println!("token: {}", token);
        });
        
        assert_eq!(3, 3);
    }

    #[test]
    fn regex_replace_all() {
        let input: &str = "camelCaseInput";
        let regex_upper: Regex = Regex::new(r"\p{Lu}").unwrap();

        let result: String = regex_upper.replace_all(input, "*").to_string();

        assert_eq!(*result, String::from("camel*ase*nput"));
    }

    #[test]
    fn regex_find_iter() {
        let input: &str = "CamelCaseInput";
        let regex_upper: Regex = Regex::new(r"\p{Lu}[a-z]+").unwrap();

        //let hash_map: HashSet<&str> = 
        
        let fmt_test = format!("test {}", 1);

        let result = regex_upper.find_iter(input)
            .map(|token| token.as_str())
            .reduce(|a, b| {
                //format!("{} {}", a, b)
                [a, b].join(" ")
            }).unwrap();

        assert_eq!(result, "Camel Case Input");
    }

    // #[test]
    // fn test_solution() {
    //     assert_eq!(solution("camelCasing"), "camel Casing");
    //     assert_eq!(solution("camelCasingTest"), "camel Casing Test");
    // }
}
