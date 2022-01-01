//https://www.codewars.com/kata/550498447451fbbd7600041c

#[cfg(test)]
mod tests {
    #[test]
    fn test_vector_contains() {
        let input = vec![121, 144, 19, 161];

        assert_eq!(input.contains(&19), true);
    }

    #[test]
    fn test_vector_bruteforce() {
        let lhs = vec![121, 144, 19, 161];
        let rhs = vec![121*121, 144*144, 161*161, 19*19];

        let mut parity = true;

        lhs.into_iter().for_each(|x| {
            if !rhs.contains(&(x*x)) {
                parity = false;
            }
        });

        assert_eq!(parity, true);
    }

    // fn comp(a: Vec<i64>, b: Vec<i64>) -> bool {
    //     // your code
    // }

    // fn testing(a: Vec<i64>, b: Vec<i64>, exp: bool) -> () {
    //     assert_eq!(comp(a, b), exp)
    // }
    
    // #[test]
    // fn tests_comp() {
    
    //     let a1 = vec![121, 144, 19, 161, 19, 144, 19, 11];
    //     let a2 = vec![11*11, 121*121, 144*144, 19*19, 161*161, 19*19, 144*144, 19*19];
    //     testing(a1, a2, true);
    //     let a1 = vec![121, 144, 19, 161, 19, 144, 19, 11];
    //     let a2 = vec![11*21, 121*121, 144*144, 19*19, 161*161, 19*19, 144*144, 19*19];
    //     testing(a1, a2, false);
    
    // }
}
