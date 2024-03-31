// https://www.codewars.com/kata/5a8d2bf60025e9163c0000bc

/**
  use itertools::{Itertools, repeat_n};
  use std::cmp::Reverse;
  
  fn solve(xs: &[i32]) -> Vec<i32> {
      xs.iter().copied().counts().into_iter()
          .sorted_by_key(|&(x, f)| (Reverse(f), x))
          .flat_map(|(x, f)| repeat_n(x, f))
          .collect()
  }
 */

use std::{cmp::Ordering, collections::HashMap};

fn solve(input: &[i32]) -> Vec<i32> {
  let mut freq_map: HashMap<i32, usize> = HashMap::new();
  input.iter().for_each(|x| {
    if !freq_map.contains_key(x) {
      freq_map.insert(*x, 0);
    }

    *freq_map.get_mut(x).unwrap() += 1;
  });

  let mut cloned = Vec::from(input);

  cloned.sort_by(|a, b| {
    let freq_a = freq_map.get(a).unwrap();
    let freq_b = freq_map.get(b).unwrap();

    match freq_a.cmp(freq_b) {
      Ordering::Less => Ordering::Greater,
      Ordering::Greater => Ordering::Less,
      Ordering::Equal => a.cmp(&b),
    }
  });

  cloned
}

#[test]
fn test_case() {
  assert_eq!(solve(&vec![2,3,5,3,7,9,5,3,7]), vec![3,3,3,5,5,7,7,2,9]);
  assert_eq!(solve(&vec![1,2,3,0,5,0,1,6,8,8,6,9,1]), vec![1,1,1,0,0,6,6,8,8,2,3,5,9]);
  assert_eq!(solve(&vec![5,9,6,9,6,5,9,9,4,4]), vec![9,9,9,9,4,4,5,5,6,6]);
  assert_eq!(solve(&vec![4,4,2,5,1,1,3,3,2,8]), vec![1,1,2,2,3,3,4,4,5,8]);
  assert_eq!(solve(&vec![4,9,5,0,7,3,8,4,9,0]), vec![0,0,4,4,9,9,3,5,7,8]);
}

#[test]
fn test_ordering() {
  let left: usize = 3;
  let right: usize = 5;

  let cmp_result = left.cmp(&right);

  assert_eq!(cmp_result.is_ne(), true);
}

#[test]
fn test_hashset() {
  let input = vec![2,3,5,3,7,9,5,3,7];

  let mut test_map: HashMap<u32, usize> = HashMap::new();

  input.iter().for_each(|x| {
    if !test_map.contains_key(x) {
      test_map.insert(*x, 0);
    }

    *test_map.get_mut(x).unwrap() += 1;
  });

  assert_eq!(test_map.get(&3).unwrap(), &3);
}

