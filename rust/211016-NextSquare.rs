//https://www.codewars.com/kata/56269eb78ad2e4ced1000013

#[cfg(test)]
mod tests {
    fn find_next_square(sq: u64) -> Option<u64> {
        let to_sqrt: f64 = (sq as f64).sqrt();
        let to_abs: f64 = to_sqrt.floor();
    
        if to_sqrt > to_abs {
            return None
        }
    
        Some ( (to_abs as u64 + 1).pow(2) )
    }

    #[test]
    fn sample_tests() {
        assert_eq!(find_next_square(121), Some(144));
        assert_eq!(find_next_square(625), Some(676));
        assert_eq!(find_next_square(319_225), Some(320_356));
        assert_eq!(find_next_square(15_241_383_936), Some(15_241_630_849));
        assert_eq!(find_next_square(155), None);
        assert_eq!(find_next_square(342_786_627), None);
    }
}
