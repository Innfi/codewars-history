// https://www.codewars.com/kata/51f1342c76b586046800002a

/*
fn solution(n: f64) -> f64 {
    (2.0 * n).round() / 2.0
}
*/

#[cfg(test)]
mod tests {
    #[test]
    fn test_mathcalls() {
        let n: f64 = 4.9;

        assert_eq!(n.floor(), 4.0);
        assert_eq!(n.ceil(), 5.0);

        assert_eq!(n.round(), 5.0);

        let n2: f64 = 4.5;
        assert_eq!(n2.round(), 5.0);
    }

    #[test]
    fn test_farray() {
        let n: f64 = 4.2;
        let floored = n.floor();
        let input = vec![ floored, floored+0.5, floored+1.0 ];

        let ret = input.into_iter().map(|x| {
            ((x-n).abs(), x)
        }).min_by(|x, y| (x.0).partial_cmp(&(y.0)).unwrap());

        assert_eq!((ret.unwrap()).1, 4.0);
    }

    fn solution(n: f64) -> f64 {
        let floored = n.floor();
        let input = vec![ floored, floored+0.5, floored+1.0 ];

        let ret = input.into_iter().map(|x| ((x-n).abs(), x))
            .min_by(|x, y| (x.0).partial_cmp(&(y.0)).unwrap());

        (ret.unwrap()).1
    }
    #[test]
    fn sample_tests() {
        assert_eq!(solution(4.2), 4.0);
        assert_eq!(solution(4.4), 4.5);
        assert_eq!(solution(4.6), 4.5);
        assert_eq!(solution(4.8), 5.0);
    }
}
