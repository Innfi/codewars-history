//https://www.codewars.com/kata/59f44c7bd4b36946fd000052

/* 
fn hist(s: &str) -> String {
    ["u", "w", "x", "z"]
        .iter() 
        .fold(vec![], |mut acc, error| {
          let count = s.matches(error).count();
          if count > 0 {
              acc.push(format!("{:2} {:<6}{}", error, count, "*".repeat(count)))
          }
          acc
        })
        .join("\r")
}
*/

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    #[test]
    fn string_filter() {
        let input: &str = "aaaccbbcddaaa";

        let output: Vec<char> = input.to_lowercase().chars().filter(|x| {
            match x {
                &'c' | &'d' => true,
                _ => false
            }
        }).collect();
        let output_string: String = output.into_iter().collect();

        assert_eq!(output_string.as_str(), "cccdd");
    }

    #[test]
    fn print_stars() {
        let output: String = (0..5).map(|_x| '*').collect::<String>();

        assert_eq!(output.as_str(), "*****");
    }

    fn hist(s: &str) -> String {
        let mut items: Vec<(char, usize)> = {
            let mut counter = HashMap::new();
            for letter in s.to_lowercase().chars().filter(|x| {
                match x {
                    &'u' | &'w' | &'x' | &'z' => true,
                    _ => false,
                }
            }) {
                let c = counter.entry(letter).or_default();
                *c += 1;
            }
            counter.drain().collect()
        };
        items.sort_unstable();
        items.into_iter().map(|(l, c)| {
            format!("{}  {}     {}",
                l,
                c,
                (0..c).map(|_x| '*').collect::<String>(),
            )
        }).collect::<Vec<_>>().join("\r")
    }

    #[test]
    fn test_runner() {
        let input = &"tpwaemuqxdmwqbqrjbeosjnejqorxdozsxnrgpgqkeihqwybzyymqeazfkyiucesxwutgszbenzvgxibxrlvmzihcb";

        let output = hist(input);

        println!("output: {}", output);
    }

    fn dotest(s: &str, exp: &str) -> () {
        println!("s:{:?}", s);   
        let ans = hist(s);
        println!("actual: {:?}", ans);
        println!("expect: {:?}", exp);
        println!("{}", ans == exp);
        assert_eq!(ans, exp);
        println!("{}", "-");
    }   

    #[test]
    fn basic_tests() {
        dotest("tpwaemuqxdmwqbqrjbeosjnejqorxdozsxnrgpgqkeihqwybzyymqeazfkyiucesxwutgszbenzvgxibxrlvmzihcb", 
                "u  3     ***\rw  4     ****\rx  6     ******\rz  6     ******");
        dotest("aaifzlnderpeurcuqjqeywdq", "u  2     **\rw  1     *\rz  1     *");
    }
}
