//https://www.codewars.com/kata/56747fd5cb988479af000028

/*
fn get_middle(s:&str) -> &str {
    &s[(s.len()-1)/2..s.len()/2+1]
}
*/

#[cfg(test)]
mod tests {
    #[test]
    fn test_strlen() {
        let input: &str = "abc";
        assert_eq!(input.len(), 3);
    }

    #[test]
    fn test_substr() {
        let input: &str = "abcd";

        let element: char = input.chars().nth(1).unwrap();
        assert_eq!(element, 'b');
    }

    #[test]
    fn test_slice() {
        let input: &str = "abcde";
        let sliced = &input[2..4];

        assert_eq!(sliced, "cd");
    }

    #[test]
    fn test_match() {
        let input: &str = "abcde";
        let middle = input.len() / 2;

        let out = match input.len() % 2 {
            0 => &input[middle-1..middle+1],
            _ => &input[middle..middle+1] 
        };

        assert_eq!(out, "c");
    }

    fn get_middle(s:&str) -> &str {
        let middle: usize = s.len()/2;

        match s.len() % 2 {
            0 => &s[middle-1..middle+1],
            _ => &s[middle..middle+1]
        }
    }

    #[test]
    fn example_tests() {
        assert_eq!(get_middle("test"),"es");
        assert_eq!(get_middle("testing"),"t");
        assert_eq!(get_middle("middle"),"dd");
        assert_eq!(get_middle("A"),"A");
        assert_eq!(get_middle("of"),"of");
    }
}
