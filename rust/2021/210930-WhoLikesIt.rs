//https://www.codewars.com/kata/5266876b8f4bf2da9b000362

/**
 * best practices: 

    fn likes(names: &[&str]) -> String {
        match names {
            [] => format!("no one likes this"),
            [a] => format!("{} likes this", a),
            [a, b] => format!("{} and {} like this", a, b),
            [a, b, c] => format!("{}, {} and {} like this", a, b, c),
            [a, b, rest @ ..] => format!("{}, {} and {} others like this", a, b, rest.len()),
        }
    }

    fn likes(names: &[&str]) -> String {
        match names.len() {
            0 => "no one likes this".to_string(),
            1 => format!("{} likes this", names[0]),
            2 => format!("{} and {} like this", names[0], names[1]),
            3 => format!("{}, {} and {} like this", names[0], names[1], names[2]),
            l => format!("{}, {} and {} others like this", names[0], names[1], l - 2),
        }
    }
 */

#[cfg(test)]
mod tests {
    fn likes(names: &[&str]) -> String {
        let names_vec: Vec<&str> = names.to_vec();
        if names_vec.is_empty() {
            return String::from("no one likes this");
        }

        let mut result: String = String::from(*names_vec.get(0).unwrap());
        if names_vec.len() == 1 {
            return format!("{} likes this", result);
        } else if names_vec.len() == 2 {    
            result.push_str(" and ");
            result.push_str(*names_vec.get(1).unwrap());
        } else if names_vec.len() == 3 {
            result.push_str(", ");
            result.push_str(*names_vec.get(1).unwrap());
            result.push_str(" and ");
            result.push_str(*names_vec.get(2).unwrap());
        } else {
            result.push_str(", ");
            result.push_str(*names_vec.get(1).unwrap());
            result.push_str(" and ");
            result.push_str(&format!("{} others", names_vec.len()-2));
        }

        result.push_str(" like this");

        result
    }

    #[test]
    fn example_tests() {
        assert_eq!(likes(&[]), "no one likes this");
        assert_eq!(likes(&["Jacob", "Alex"]), "Jacob and Alex like this");
        assert_eq!(
            likes(&["Max", "John", "Mark"]),
            "Max, John and Mark like this"
        );
        assert_eq!(
            likes(&["Alex", "Jacob", "Mark", "Max"]),
            "Alex, Jacob and 2 others like this"
        );
    }
}
