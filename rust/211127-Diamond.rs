//https://www.codewars.com/kata/5503013e34137eeeaa001648

/**
 * fn print(n: i32) -> Option<String> {
    if n < 0 || n % 2 == 0 {
        return None;
    }

    let n = n as usize;
    let diamond = (1..=n)
        .chain((1..n).rev())
        .step_by(2)
        .map(|i| format!("{}{}\n", " ".repeat((n - i) / 2), "*".repeat(i)))
        .collect();

    Some(diamond)
}
 */

#[cfg(test)]
mod tests {
    #[test]
    fn string_format() {
        let tried: String = format!("{:<7}\n", "*");
        assert_eq!(tried, "*      \n");

        let tried2: String = format!("{:<3}{}{:<3}\n", " ", "*", " ");
        assert_eq!(tried2, "   *   \n");

        let tried3: String = format!("{:^7}\n", "*");
        assert_eq!(tried2, tried3);

        let tried4: String = format!("{:^7}\n", "***");
        assert_eq!(tried4, "  ***  \n");

        let test_string: String = format!("{star:^width$}\n", star="*", width=7);
        assert_eq!(test_string, "   *   \n");

        let test_stars: String = format!("{:*^1$}", "*", 5);
        assert_eq!(test_stars, "*****");

        let repeated: String = "*".repeat(5);
        assert_eq!(repeated, "*****");
    }

    fn print(n: i32) -> Option<String> {
        let mut current: i32 = 1;
        let mut increment = 1;
        let mut star_vec: Vec<String> = vec![];
        loop {
            if n %2 == 0 || n < 0 || current < 0 {
                break;
            } else {
                let star_string: String = "*".repeat(current as usize);
                let complete_string: String = 
                    format!("{star:^width$}", star=star_string, width=n as usize);
                star_vec.push(format!("{}\n", complete_string.trim_end()));

                if increment >= n {
                    current -= 2;
                } else {
                    current += 2;
                }

                increment += 2;
            }
        }

        match star_vec.is_empty() {
            false => Some(star_vec.into_iter().collect()), 
            _ => None
        }
    }

    #[test]
    fn basic_test() {
        assert_eq!(print(3), Some(" *\n***\n *\n".to_string()) );
        assert_eq!(print(5), Some("  *\n ***\n*****\n ***\n  *\n".to_string()) );
        assert_eq!(print(-3),None);
        assert_eq!(print(2),None);
        assert_eq!(print(0),None);
        assert_eq!(print(1), Some("*\n".to_string()) );
    }
}
