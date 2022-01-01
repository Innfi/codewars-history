//https://www.codewars.com/kata/559b8e46fa060b2c6a0000bf

/*

fn diagonal(n: u32, p: u32) -> u64 {
    let b = (1..=(p+1)).fold(1.0, |f, i| f*(i as f64));
    let a = ((n-p+1)..=(n+1)).fold(1.0/b, |f, i| f*(i as f64));
    a.round() as u64
}

*/

#[cfg(test)]
mod tests {
    fn factorial(n: u32) -> u64 {
        let mut ret: u64 = 1;
        for x in (1..n+1).rev() {
            ret = ret * x as u64;
        }

        ret
    }

    fn to_bico(n: u32, k: u32) -> u64 {
        //factorial(n) / (factorial(k) * factorial(n-k))
        let mut real_k: u32 = k;
        if k > n - k {
            real_k = n - k;
        }

        let mut ret: u64 = 1;
        for x in 1..real_k+1 {
            ret *= (n-real_k+x) as u64;
            ret /= x as u64;
        }

        ret
    }

    
    #[test]
    fn test_binomial_coefficient() {
        assert_eq!(to_bico(3, 2), 3);
        assert_eq!(to_bico(4, 2), 6);
        assert_eq!(to_bico(5, 2), 10);
        assert_eq!(to_bico(6, 2), 15);
    }

    #[test]
    fn get_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(3), 6);
        assert_eq!(factorial(7), 5040);
    }

    fn diagonal(n: u32, p: u32) -> u64 {
        let mut sum = 0;
        for x in p..n+1 {
            sum += to_bico(x, p);
        }

        sum
    }

    fn testing(n: u32, p: u32, exp: u64) -> () {
        assert_eq!(diagonal(n, p), exp)
    }
    #[test]
    fn basics_diagonal() {
        testing(20,3, 5985);
        testing(20,4, 20349);
        testing(20,5, 54264);
        testing(20,15, 20349);
        testing(55, 20, 1346766106565880);
    }
}
