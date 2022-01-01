//https://www.codewars.com/kata/593c9175933500f33400003e

/*
fn multiples(m: i32, n: f64) -> Vec<f64> {
    (1..=m).map(|d| d as f64 * n).collect()
}

*/

#[cfg(test)]
mod tests {
    fn multiples(m: i32, n: f64) -> Vec<f64> {
        let mut result: Vec<f64> = Vec::new();
    
        for x in 1..=m {
            result.push(f64::from(x) * n);
        }
    
         result
    }

    #[test]
    fn test_add() {
        assert_eq!(multiples(3, 5.0), vec![5.0, 10.0, 15.0]);
        assert_eq!(multiples(5, -1.0), vec![-1.0, -2.0, -3.0, -4.0, -5.0]);
        assert_eq!(multiples(1, 3.14), vec![3.14]);
    } 
}
