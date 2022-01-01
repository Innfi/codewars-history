//https://www.codewars.com/kata/5355a811a93a501adf000ab7

/*
fn fizz_buzz_custom_solver(string_one: &str, string_two: &str, num_one: usize, num_two: usize) -> Vec<String> {
    (1usize..=100).map(|n| match (n % num_one, n % num_two) {
        (0, 0) => string_one.to_string() + string_two,
        (0, _) => string_one.to_string(),
        (_, 0) => string_two.to_string(),
        (_, _) => n.to_string(),
    }).collect()
}

*/

#[cfg(test)]
mod tests {
    #[test]
    fn create_string_vec() {
        let test_vector: Vec<String> = (1..101).into_iter().map(|x| {
            x.to_string()
        }).collect();

        assert_eq!(test_vector.len(), 100);
    }

    #[test]
    fn test_div() {
        assert_eq!(15 % 3, 0);
        assert_eq!(15 % 5, 0);
        assert_eq!(2*37 / 2, 37);
    }

    fn fizz_buzz_custom_solver(
        string_one: &str, 
        string_two: &str, 
        num_one: usize, 
        num_two: usize
    ) -> Vec<String> {
        (1..101).into_iter().map(|x| {
            if x % num_one == 0 && x % num_two == 0 {
                return format!("{}{}", string_one, string_two);
            }
            if x % num_one == 0 {
                return String::from(string_one);
            }
            if x % num_two == 0 {
                return String::from(string_two);
            }
            
             
            x.to_string()
        }).collect()
    }
    
    #[macro_export]
    macro_rules! fizz_buzz_custom {
        () => { fizz_buzz_custom_solver("Fizz", "Buzz", 3, 5) };
        ($str_one:expr) => { 
            fizz_buzz_custom_solver($str_one, "Buzz", 3, 5) 
        };
        ($str_one:expr, $str_two:expr) => { 
            fizz_buzz_custom_solver($str_one, $str_two, 3, 5) 
        };
        ($str_one:expr, $str_two:expr, $num_one:expr) => { 
            fizz_buzz_custom_solver($str_one, $str_two, $num_one, 5) 
        };
        ($str_one:expr, $str_two:expr, $num_one:expr, $num_two:expr) => { 
            fizz_buzz_custom_solver($str_one, $str_two, $num_one, $num_two) 
        };
    }

    #[test]
    fn test_basic() {
        let test_vec: Vec<String> = fizz_buzz_custom!("up", "front", 2, 37);
        test_vec.into_iter().for_each(|x| {
            print!("{} ", x);
        });


        assert_eq!(fizz_buzz_custom!()[3], "4".to_string());
        assert_eq!(fizz_buzz_custom!()[15], "16".to_string());
    }
}
