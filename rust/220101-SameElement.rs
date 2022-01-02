//https://www.codewars.com/kata/550498447451fbbd7600041c

/*
fn comp(a: Vec<i64>, b: Vec<i64>) -> bool {
    let mut a1 = a.iter().map(|&x| x * x).collect::<Vec<_>>(); 
    let mut a2 = b;
    a1.sort();
    a2.sort();
    a1 == a2
}
*/

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

    #[test]
    fn test_sqrt() {
        let input: i64 = (144*144)-1;

        let output: f64 = (input as f64).sqrt();

        println!("output: {}", output);
        println!("output2: {}", output as i64);
        println!("output3: {}", output.floor());
    }

    fn comp(a: Vec<i64>, b: Vec<i64>) -> bool {        
        let mut b_copy = b.clone();
        
        for x in a.iter() {
            let mut b_iter = b_copy.iter_mut();    
            let pos = b_iter.position(|elem| &(x*x) == elem);
            if pos == None {
                return false;
            }

            b_copy.remove(pos.unwrap());
        }
       
        return b_copy.is_empty();
    }

    fn testing(a: Vec<i64>, b: Vec<i64>, exp: bool) -> () {
        assert_eq!(comp(a, b), exp)
    }
    
    #[test]
    fn tests_comp() {
        let a1 = vec![121, 144, 19, 161, 19, 144, 19, 11];
        let a2 = vec![11*11, 121*121, 144*144, 19*19, 161*161, 19*19, 144*144, 19*19];
        testing(a1, a2, true);
        let a1 = vec![121, 144, 19, 161, 19, 144, 19, 11];
        let a2 = vec![11*21, 121*121, 144*144, 19*19, 161*161, 19*19, 144*144, 19*19];
        testing(a1, a2, false);

        let a1 = vec![2, 2, 3];
        let a2 = vec![4, 9, 9];
        testing(a1, a2, false);
    }
}
