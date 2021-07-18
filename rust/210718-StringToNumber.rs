//https://www.codewars.com/kata/544675c6f971f7399a000e79

fn string_to_number(s: &str) -> i32 {
    // use unwrap() from Result to actual value
    s.parse::<i32>().unwrap()
}