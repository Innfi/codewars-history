// https://www.codewars.com/kata/5a3e1319b6486ac96f000049

/**
 * fn pairs(xs: &Vec<i32>) -> usize {
    xs.chunks_exact(2).filter(|xy| (xy[0] - xy[1]).abs() == 1).count()
}
 */

#[cfg(test)]
mod tests {
  fn pairs(arr: &Vec<i32>) -> usize {
    let mut sum: usize = 0;

    let mut temp: &i32 = &0;
    for (index, elem) in arr.iter().enumerate() {
      if index %2 == 0 {
        temp = elem;
        continue;
      }

      if (temp-elem).abs() == 1 {
        sum += 1;
        temp = &0;
      }
    }

    sum
  }

  #[test]
  fn returns_expected() {
    assert_eq!(pairs(&vec![1, 2, 5, 8, -4, -3, 7, 6, 5]), 3);
  }
}