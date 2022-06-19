//https://www.codewars.com/kata/59590976838112bfea0000fa

/*
fn beggars(values: &[u32], n: usize) -> Vec<u32> {
    (0..n).map(|i| values.iter().skip(i).step_by(n).sum()).collect()
}
*/

#[cfg(test)]
mod tests {
    #[test]
    fn fill_vector() {
        let len = 10;
        let default_value = 0;

        let input = vec![default_value; len];

        assert_eq!(input.len(), len);
        input.into_iter().for_each(|x| {
            assert_eq!(x, 0);
        });
    }

    #[test]
    fn modify_vector_element() {
        let mut input = vec![0; 5];
        input[1] = 1;

        assert_eq!(input[1], 1);
    }

    fn beggars(values: &[u32], n: usize) -> Vec<u32> {
        let mut out_vec = vec![0; n];
        if n < 1 { return out_vec; }

        for i in 0..values.len() {
            out_vec[i%n] += values[i];
        }
        
        out_vec
    }

    #[test]
    fn test_basic() {
        assert_eq!(beggars(&[1, 2, 3, 4, 5], 1), [15]);
        assert_eq!(beggars(&[1, 2, 3, 4, 5], 2), [9, 6]);
        assert_eq!(beggars(&[1, 2, 3, 4, 5], 3), [5, 7, 3]);
        assert_eq!(beggars(&[1, 2, 3, 4, 5], 6), [1, 2, 3, 4, 5, 0]);
    }
    
    #[test]
    fn test_zero_beggars() {
        assert_eq!(beggars(&[1, 2, 3, 4, 5], 0), []);
    }
}
