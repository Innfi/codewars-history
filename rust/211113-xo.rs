//https://www.codewars.com/kata/55908aad6620c066bc00002a

#[cfg(test)]
mod tests {
    #[test]
    fn test_tolower() {
        assert_eq!("xOXO".to_lowercase(), "xoxo");
    }

    #[test]
    fn test_sum() {
        let input = vec![ 1, 2, 3, 4 ];
        assert_eq!(input.into_iter().sum::<i32>(), 10);
    }

    fn xo(string: &'static str) -> bool {
        string.to_lowercase().chars().map(|x| {
            match x {
                'x' => 1, 
                'o' => -1, 
                _ => 0
            }
        }).sum::<i32>() == 0
    }

    #[test]
    fn returns_expected() {
        assert_eq!(xo("xo"), true);
    }
}
