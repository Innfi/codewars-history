//https://www.codewars.com/kata/52a78825cdfc2cfc87000005

#[cfg(test)]
mod tests {    
    #[test]
    fn test_parse_negative() {
        let input: String = format!("-5.0");

        let out: f64 = input.parse().unwrap();

        assert_eq!(out, -5.0);
    }

    #[test]
    fn parse_expression() {
        let input: &str = "1.0 - -6.0";
        let input_vec: Vec<char> = input.chars().collect();
        let mut number_vec: Vec<char> = Vec::new();

        let mut token_vec: Vec<String> = Vec::new();
        let expected_result: Vec<String> = vec![
            String::from("1.0"),
            String::from("-"),
            String::from("-6.0"),
        ];

        input_vec.iter().enumerate().for_each(|(index, x)| {
            if x.is_digit(10) || x == &'.' {
                number_vec.push(*x);
            } else {
                if x == &'-' && input_vec[index+1].is_digit(10) {
                    number_vec.push(*x);
                    return;
                }

                if !number_vec.is_empty() {
                    let number_string: String = number_vec.iter().collect();
                    token_vec.push(number_string);
                    number_vec.clear();
                }

                if x != &' ' { token_vec.push(format!("{}", x)); }
            }
        });

        if !number_vec.is_empty() {
            let number_string: String = number_vec.iter().collect();
            token_vec.push(number_string);
            number_vec.clear();
        }

        assert_eq!(token_vec, expected_result);
    }

    struct ParseResult {
        ops: Vec<char>,
        numbers: Vec<f64>,
    }

    fn parse_expr(expr: &str) -> ParseResult {
        let mut ops_vec: Vec<char> = Vec::new();
        let mut numbers_vec: Vec<f64> = Vec::new();

        let mut partial_numbers: Vec<char> = Vec::new();

        let input_vec:Vec<char> = expr.chars().collect();
        input_vec.iter().enumerate().for_each(|(index, x)| {
            if x.is_digit(10) || x == &'.' {
                partial_numbers.push(*x);
            } else {
                if x == &'-' && input_vec[index+1].is_digit(10) {
                    partial_numbers.push(*x);
                    return;
                }

                if !partial_numbers.is_empty() {
                    let number_string: String = partial_numbers.iter().collect();
                    let to_number: f64 = number_string.parse().unwrap();
                    numbers_vec.push(to_number);

                    partial_numbers.clear();
                }

                if x != &' ' { ops_vec.push(*x); }
            }
        });

        if !partial_numbers.is_empty() {
            let number_string: String = partial_numbers.iter().collect();
            let to_number: f64 = number_string.parse().unwrap();
            numbers_vec.push(to_number);

            partial_numbers.clear();
        }

        ParseResult {
            ops: ops_vec,
            numbers: numbers_vec,
        }
    }

    // fn calc(expr: &str) -> f64 {
        
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
