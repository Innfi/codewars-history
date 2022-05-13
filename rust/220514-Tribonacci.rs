//https://www.codewars.com/kata/556deca17c58da83c00002db

#[cfg(test)]
mod tests {
    #[test]
    fn test_array_indexing() {
        let input: &[f64; 4] = &[0., 1., 1., 2.];
        let subset: &[f64] = &input[1..4];

        let sum = subset.iter().sum::<f64>();

        assert_eq!(sum, 4.);
    }

    #[test]
    fn test_mut_vector_indexing() {
        let mut input: Vec<f64> =  [0., 1., 1., 2.].to_vec();
        
        input.push(*(&input[1..4].iter().sum::<f64>()));
        let last = input.into_iter().last().unwrap();

        assert_eq!(last, 4.);
    }

    fn tribonacci(signature: &[f64; 3], n: usize) -> Vec<f64> {
        let mut output: Vec<f64> = (*signature).to_vec();

        if n < 4 { return output[0..n].to_vec(); }

        for x in 0..n-3 {
            output.push(output[x..x+3].iter().sum::<f64>());
        }

        output
    }

    #[test]
    fn basic_tests() {
        assert_eq!(tribonacci(&[0., 1., 1.], 10), vec![0., 1., 1., 2., 4., 7., 13., 24., 44., 81.]);
        assert_eq!(tribonacci(&[1., 0., 0.], 10), vec![1., 0., 0., 1., 1., 2., 4., 7., 13., 24.]);
        assert_eq!(tribonacci(&[0., 0., 0.], 10), vec![0., 0., 0., 0., 0., 0., 0., 0., 0., 0.]);
        assert_eq!(tribonacci(&[1., 2., 3.], 10), vec![1., 2., 3., 6., 11., 20., 37., 68., 125., 230.]);
        assert_eq!(tribonacci(&[3., 2., 1.], 10), vec![3., 2., 1., 6., 9., 16., 31., 56., 103., 190.]);
        assert_eq!(tribonacci(&[1., 1., 1.], 1), vec![1.]);
        assert_eq!(tribonacci(&[300., 200., 100.], 0), vec![]);
        assert_eq!(tribonacci(&[0.5, 0.5, 0.5], 30), vec![0.5, 0.5, 0.5, 1.5, 2.5, 4.5, 8.5, 15.5, 28.5, 52.5, 96.5, 177.5, 326.5, 600.5, 1104.5, 2031.5, 3736.5, 6872.5, 12640.5, 23249.5, 42762.5, 78652.5, 144664.5, 266079.5, 489396.5, 900140.5, 1655616.5, 3045153.5, 5600910.5, 10301680.5]);;
    }
}
