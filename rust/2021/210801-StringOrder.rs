#[cfg(test)]
mod tests {
    use std::str;
    use regex::Regex;

    fn order(sentence: &str) -> String {
        let mut vec: Vec<&str> = sentence.split_whitespace().collect();

        vec.sort_by(|a, b| to_number(a).cmp(  &(to_number(b))   ));

        vec.join(" ")
    }

    fn to_number(input: &str) -> i32 {
        let result = Regex::new(r"[0123456789]").unwrap().captures(input).unwrap();

        *( &result[0].parse::<i32>().unwrap() )
    }

    fn print_elements(mut vec: Vec<&str>) {
        vec.sort();
        let iter = vec.iter();
        
        for val in iter {
            println!("{}", val);
        }
    }

    #[test]
    fn find_number_in_string() {
        let input: &str = "T3st";

        let re = Regex::new(r"[0123456789]").unwrap();

        //let result = re.replace_all(input, "Z");
        //assert_eq!(result, "TZst");

        let result = re.captures(input).unwrap();
        let result_as_number = &result[0].parse::<i32>().unwrap();

        assert_eq!(*result_as_number, 3);
    }

    #[test]
    fn string_split() {
        let input: &str = "is2 Thi1s T4est 3a";

        let splitted: str::Split<&str> = input.split(" ");
        let vec: Vec<&str> = splitted.collect();

        print_elements(vec);
    }

    #[test]
    fn string_split_whitespace() {
        let input: &str = "a b c";
        let splitted = input.split_whitespace().map(|s| s);
        
        assert_eq!(splitted.last().unwrap(), "c");
    }

    #[test]
    fn returns_expected() {
        assert_eq!(order("is2 Thi1s T4est 3a"), "Thi1s is2 3a T4est");
        assert_eq!(order(""), "");
    }
}
