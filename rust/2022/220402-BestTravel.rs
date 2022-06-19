//https://www.codewars.com/kata/55e7280b40e1c4a06d0000aa

/*
extern crate itertools;
use itertools::Itertools;

fn choose_best_sum(t: i32, k: i32, ls: &Vec<i32>) -> i32 {
    ls
        .iter()
        .combinations(k as usize)
        .map(|comb| comb.into_iter().sum())
        .filter(|s| s <= &t)
        .max()
        .unwrap_or(-1)
}
*/
#[cfg(test)]
mod tests {
    fn dp_subroutine(
        t: i32,
        k: i32,
        ls: &Vec<i32>,
        index: usize,
        sum: &i32,
        sum_count: i32,
        ) -> (i32, i32) {

        if sum_count >= k { return (*sum, sum_count); }
        if index+1 >= ls.len() { return (*sum, sum_count); }

        let mut dp_output: (i32, i32) = (0, 0);

        for i in index+1..ls.len() {
            let output = dp_subroutine(t, k, ls, i, &(sum+ls[i]), sum_count+1);
            if output.0 <= t && dp_output.0 < output.0 { dp_output = output; }
        }

        dp_output
    }

    #[test]
    fn dp_iterating() {
        let t: i32 = 174;
        let k: i32 = 3;

        let dists = &vec![50, 55, 57, 58, 60];
        let mut best = 0;
        for (i, x) in dists.iter().enumerate() {
            let dp_output = dp_subroutine(t, k, dists, i, x, 1);
            println!("dp_output: {}, {}", dp_output.0, dp_output.1);

            if dp_output.0 <= t && best < dp_output.0 && dp_output.1 == k {
                best = dp_output.0;
            }
            //if dp_output < t && best < dp_output { best = dp_output; }
        }

        assert_eq!(best, 173);
    }

    fn choose_best_sum(t: i32, k: i32, ls: &Vec<i32>) -> i32 {
        if k as usize > ls.len() { return -1; }
        let mut best_sum = -1;

        for i in 0..ls.len() {
            let dp_output = dp_subroutine(t, k, ls, i, &ls[i], 1);
            if dp_output.0 <= t && best_sum < dp_output.0 && dp_output.1 == k {
                best_sum = dp_output.0;
            }
        }

        best_sum
    }

    fn testing(t: i32, k: i32, ls: &Vec<i32>, exp: i32) -> () {
        assert_eq!(choose_best_sum(t, k, ls), exp)
    }
    
    #[test]
    fn basics_choose_best_sum() {
        let ts = &vec![50, 55, 56, 57, 58];
        testing(163, 3, ts, 163);
        let ts = &vec![50];
        testing(163, 3, ts, -1);
        let ts = &vec![91, 74, 73, 85, 73, 81, 87];
        testing(230, 3, ts, 228);
        testing(331, 2, ts, 178);

        let ts = &vec![91, 74, 73, 85, 73, 81, 87];
        testing(331, 5, ts, -1);
    }
}
