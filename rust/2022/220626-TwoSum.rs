//https://www.codewars.com/kata/52c31f8e6605bcc646000082

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    fn two_sum(numbers: &[i32], target: i32) -> (usize, usize) {
        let mut input_map: HashMap<i32, Vec<usize>> = HashMap::new();
        for i in 0..numbers.len() {
            if !input_map.contains_key(&numbers[i]) {
                let input_vec = vec![i];
                input_map.insert(numbers[i], input_vec);

                continue;
            }

            let input_vec = input_map.get_mut(&numbers[i]).unwrap();
            input_vec.push(i);
        }

        for i in 0..numbers.len() {
            let diff = target - numbers[i];

            if !input_map.contains_key(&diff) { continue; }

            let input_vec = input_map.get(&diff).unwrap();
            for j in 0..input_vec.len() {
                if i != input_vec[j] { return (i, input_vec[j]); }
            }
        }

        (0, 0)
    }

    fn two_sum1(numbers: &[i32], target: i32) -> (usize, usize) {
        for i in 0..numbers.len() {
            let diff = target - numbers[i];

            for j in i+1..numbers.len() {
                if diff == numbers[j] { return (i, j); }
            }
        }

        (0, 0)
    }

    #[test]
    fn sample() {
        do_test(&[1, 2, 3], 4);
        do_test(&[1234, 5678, 9012], 14690);
        do_test(&[2, 2, 3], 4);
    }
    
    fn do_test(nums: &[i32], sum: i32) {
        let len = nums.len();
        let user_tuple = two_sum(nums, sum);
        assert!(
            user_tuple.0 < len && user_tuple.1 < len,
            "\nnumbers: {:?}\ntarget: {}\nresult: {:?}\nresult tuple has an index out of bounds",
            nums, sum, user_tuple 
        );
        assert!(
            user_tuple.0 != user_tuple.1,
            "\nnumbers: {:?}\ntarget: {}\nresult: {:?}\nresult tuple must have two different indices",
            nums, sum, user_tuple 
        );
        let num1 = nums[user_tuple.0];
        let num2 = nums[user_tuple.1];
        let user_sum = num1 + num2;
        assert!(
            user_sum == sum,
            "\nnumbers: {:?}\ntarget: {}\nresult: {:?}\nnumber as index {}: {}\nnumber as index {}: {}\nsum of the two numbers: {}\nsum of the two numbers did not equal target",
            nums, sum, user_tuple, user_tuple.0, num1, user_tuple.1, num2, user_sum
        )
    }
}
