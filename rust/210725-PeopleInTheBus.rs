#[cfg(test)]
mod tests {
    fn number(bus_stops:&[(i32,i32)]) -> i32 {
        bus_stops.iter().map(|(x, y)| x-y).reduce(|a, b| a+b).unwrap()
    }

    #[test]
    fn returns_expected() {
        assert_eq!(number(&[(10,0),(3,5),(5,8)]), 5);
        assert_eq!(number(&[(3,0),(9,1),(4,10),(12,2),(6,1),(7,10)]), 17);
        assert_eq!(number(&[(3,0),(9,1),(4,8),(12,2),(6,1),(7,8)]), 21);
    }

    #[test]
    fn test_array() {
        let input = &[(10, 0), (3, 5), (5, 8)];
        let output = input.iter().map(|(x, y)| x-y).reduce(|a, b| a+b);

        assert_eq!(output.unwrap(), 5);
    }
}
