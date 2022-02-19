//https://www.codewars.com/kata/5265326f5fda8eb1160004c8

#[cfg(test)]
mod tests {
    fn number_to_string(i: i32) -> String {
        format!("{}", i)
    }

    #[test]
    fn returns_expected() {
        assert_eq!(number_to_string(67), "67");
        assert_eq!(number_to_string(1+2), "3");
    }
}
