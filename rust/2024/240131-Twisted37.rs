// https://www.codewars.com/kata/58068479c27998b11900056e

/**
 * use itertools::Itertools;

  fn sort_twisted_37(list: &[i8]) -> Vec<i8> {
      list.iter().sorted_by_key(|x| x.to_string().chars().map(|x| 
        match x{
            '3' => '7',
            '7' => '3',
              _ => x
          }
        ).collect::<String>().parse::<i64>().unwrap()
      ).cloned().collect()
  }
 */

#[cfg(test)]
mod tests {
  fn flip(input: &i8) -> i8 {
    input.to_string().chars().map(|elem| {
      if elem == '3' {
        return '7';
      }
      if elem == '7' {
        return '3';
      }
      elem
    }).collect::<String>().parse::<i8>().unwrap()
  }

  fn sort_twisted_37(list: &[i8]) -> Vec<i8> {
    let mut input: Vec<i8> = list.into();

    input.sort_by(|a, b| {
      let new_a = flip(a);
      let new_b = flip(b);     
      new_a.cmp(&new_b)
    });
    input
  }

  #[test]
  fn test_twisted_37() {
    assert_eq!(sort_twisted_37(&[12, 13, 14]), [12, 14, 13]);
    assert_eq!(sort_twisted_37(&[9, 2, 4, 7, 3]), [2, 7, 4, 3, 9]);
  }

  #[test]
  fn mut_array() {
    let mut input: Vec<i8> = vec![1, 2, 3, 4];

    assert_eq!(input[1], 2);

    input[2] = 7;

    assert_eq!(input.len(), 4);
    assert_eq!(input[2], 7);
  }

  #[test]
  fn sort_array() {
    let mut input: Vec<i8> = vec![5, 3, 8, 1];

    input.sort();
    assert_eq!(input, vec![1, 3, 5, 8]);
  }

  #[test]
  fn remainders() {
    assert_eq!(103 % 10, 3);
    assert_eq!(327 % 10, 7);
    assert_eq!(3 % 10, 3);
    assert_eq!(7 % 10, 7);
  }
}

