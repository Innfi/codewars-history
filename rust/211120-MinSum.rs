//https://www.codewars.com/kata/5a523566b3bfa84c2e00010b

#[cfg(test)]
mod tests {
    #[test]
    fn test_sort() {
        let input: &[u64] = &[5, 4, 2, 3];
        let mut input_vec: Vec<u64> = input.to_vec();
        input_vec.sort();

        let output: &[u64] = &[2,3,4,5];
        let result: Vec<u64> = output.to_vec();

        assert_eq!(input_vec, result);
    }

    fn min_sum(xs: &[u64]) -> u64 {
        let mut xs_vec: Vec<u64> = xs.to_vec();
        xs_vec.sort();
        
        let mut ret: u64 = 0;
        for i in 0..xs_vec.len() / 2 {
            ret += xs_vec[i] * xs_vec[xs_vec.len()-1-i];
        }

        ret
    }

    #[test]
    fn basic() {
        assert_eq!(min_sum(&[5,4,2,3]), 22);
        assert_eq!(min_sum(&[12,6,10,26,3,24]), 342);
        assert_eq!(min_sum(&[9,2,8,7,5,4,0,6]), 74);        
    }      
}
