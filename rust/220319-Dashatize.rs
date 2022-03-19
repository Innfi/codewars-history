//https://www.codewars.com/kata/58223370aef9fc03fd000071

/*
 fn dashatize(n: i64) -> String {
    n.abs()
        .to_string()
        .replace("1", "-1-")
        .replace("3", "-3-")
        .replace("5", "-5-")
        .replace("7", "-7-")
        .replace("9", "-9-")
        .replace("--", "-")
        .trim_matches('-')
        .to_owned()
}
*/

#[cfg(test)]
mod tests {
    use regex::Regex;

    #[test]
    fn integer_to_array() {
        let n: i64 = 974302;
        let n_string: String = format!("{}", n);

        assert_eq!(n_string, String:: from("974302"));
    }

    #[test]
    fn using_regex() {
        let re = Regex::new(r"([13579])").unwrap();
        let text = String::from("974302");

        let after = re.replace_all(text.as_str(), "-$1-");

        assert_eq!(after, "-9--7-4-3-02");
    }

    #[test]
    fn using_regex2() {
        let text = "11111";

        let after = Regex::new(r"([13579])").unwrap()
            .replace_all(&text, "-$1-");
        let after2 = Regex::new(r"(--)").unwrap()
            .replace_all(&after, "-");
        let output = String::from(Regex::new(r"(^-|-$)").unwrap()
        .replace_all(&after2, ""));

        assert_eq!(output.as_str(), "1-1-1-1-1");
    }

    #[test]
    fn negative_to_string() {
        let input = -1;
        let input_str = format!("{}", input);

        assert_eq!(input_str, String::from("-1"));
    }

    fn dashatize(n: i64) -> String {
        let n_str: String = n.to_string();
        let first = Regex::new(r"([13579])").unwrap()
            .replace_all(&n_str, "-$1-");
        let second = Regex::new(r"(--)").unwrap()
            .replace_all(&first, "-");
        String::from(Regex::new(r"(^-|-$)").unwrap()
            .replace_all(&second, ""))
    }

    #[test]
    fn basic() {
        assert_eq!(dashatize(274), "2-7-4");
        assert_eq!(dashatize(5311), "5-3-1-1");
        assert_eq!(dashatize(86320), "86-3-20");
        assert_eq!(dashatize(974302), "9-7-4-3-02");
    }
    
    #[test]
    fn weird() {
        assert_eq!(dashatize(0), "0");
        assert_eq!(dashatize(-1), "1");
        assert_eq!(dashatize(-28369), "28-3-6-9");                
    }
}
