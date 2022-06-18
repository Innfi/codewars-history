//https://www.codewars.com/kata/52a78825cdfc2cfc87000005

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_float() {
        let input: &str = "1.3 + test";

        let mut number_vec: Vec<char> = Vec::new();
        let mut to_number: f64 = 0.0;

        input.chars().for_each(|x: char| {
            if x.is_digit(10) || x == '.' { 
                number_vec.push(x);
            } else {
                if !number_vec.is_empty() {
                    let number_string: String = number_vec.iter().collect();
                    to_number = number_string.parse().unwrap();
                    number_vec.clear();
                }
            }
        });

        assert_eq!(to_number, 1.3);
    }

    #[test]
    fn parse_basic_expression() {
        let input: &str = "1.3 + 4.1";

        // let mut token_vec
    }

    // fn calc(expr: &str) -> f64 {
    //     todo!()
    // }

    // macro_rules! assert_expr_eq {
    //     ($expr: expr, $expect: expr) => {
    //         assert_eq!(
    //             calc($expr),
    //             $expect,
    //             "\nexpected expression \"{}\" to equal \"{:?}\", but got \"{:?}\"",
    //             $expr,
    //             $expect,
    //             calc($expr),
    //         );
    //     }
    // }
    
    // #[test]
    // fn single_values() {
    //     assert_expr_eq!("0", 0.0);
    //     assert_expr_eq!("1", 1.0);
    //     assert_expr_eq!("42", 42.0);
    //     assert_expr_eq!("350", 350.0);
    // }

    // #[test]
    // fn basic_operations() {
    //     assert_expr_eq!("1 + 1", 2.0);
    //     assert_expr_eq!("1 - 1", 0.0);
    //     assert_expr_eq!("1 * 1", 1.0);
    //     assert_expr_eq!("1 / 1", 1.0);
    //     assert_expr_eq!("12 * 123", 1476.0);
    // }

    // #[test]
    // fn whitespace_between_operators_and_operands() {
    //     assert_expr_eq!("1-1", 0.0);
    //     assert_expr_eq!("1 -1", 0.0);
    //     assert_expr_eq!("1- 1", 0.0);
    //     assert_expr_eq!("1* 1", 1.0);
    // }

    // #[test]
    // fn unary_minuses() {
    //     assert_expr_eq!("1- -1", 2.0);
    //     assert_expr_eq!("1--1", 2.0);
    //     assert_expr_eq!("1 - -1", 2.0);
    //     assert_expr_eq!("-42", -42.0);
    // }

    // #[test]
    // fn parentheses() {
    //     assert_expr_eq!("(1)", 1.0);
    //     assert_expr_eq!("((1))", 1.0);
    //     assert_expr_eq!("((80 - (19)))", 61.0);
    // }
}
