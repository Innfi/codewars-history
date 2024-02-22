// https://www.codewars.com/kata/5946a0a64a2c5b596500019a

/**
 * fn split_and_add(arr: &[u32], n: usize) -> Vec<u32> {
    (0..n).fold(arr.to_vec(), |v,_| {let l=v.len(); let h=l/2; (h..l).map(|i| v[i]+if i+h<l {0} else {v[i+h-l]}).collect::<Vec<_>>()})
}
 */

#[cfg(test)]
mod tests {
  fn split_and_add(arr: &[u32], n: usize) -> Vec<u32> {
    let mut output: Vec<u32> = arr.into();

    for _ in 0..n {
      if output.len() <= 1 {
        return output;
      }
      if output.len() % 2 == 1 {
        output.insert(0, 0);
      }

      let len = output.len();
      output = fold_vec(output, len/2);
    }

    output
  }

  fn fold_vec(input: Vec<u32>, index: usize) -> Vec<u32> {
    input[0..index]
    .iter()
    .zip(input[index..index*2].iter())
    .map(|(a, b)| a+b).collect()
  }

  #[test]
  fn test_cases() {
    assert_eq!(split_and_add(&[1,2,3,4,5], 1), vec![3,5,7]);
    assert_eq!(split_and_add(&[1,2,3,4,5], 2), vec![5,10]);
    assert_eq!(split_and_add(&[1,2,3,4,5], 3), vec![15]);
    assert_eq!(split_and_add(&[15], 3), vec![15]);
    assert_eq!(split_and_add(&[32,45,43,23,54,23,54,34], 2), vec![183, 125]);
    assert_eq!(split_and_add(&[32,45,43,23,54,23,54,34], 0), vec![32,45,43,23,54,23,54,34]);
    assert_eq!(split_and_add(&[3,234,25,345,45,34,234,235,345], 3), vec![305, 1195]);
    assert_eq!(split_and_add(&[3,234,25,345,45,34,234,235,345,34,534,45,645,645,645,4656,45,3], 4), vec![1040, 7712]);
    assert_eq!(split_and_add(&[23,345,345,345,34536,567,568,6,34536,54,7546,456], 20), vec![79327]);
  }

  #[test]
  fn test_split_array() {
    let input = &[1,2,3];

    assert_eq!(input.len(), 3);
    assert_eq!(input.len()/2, 1);
  }

  #[test]
  fn test_split_array2() {
    let input = &[1,2,3,4,5,6,7,8,9];

    assert_eq!(input.len()/2, 4);
    assert_eq!(input.iter().nth(4).unwrap().eq(&5), true);
  }

  #[test]
  fn test_vector_chunks() {
    let test_arr: &[u32] = &[0,1,2,3,4,  5,6,7,8,9];
    let left = &test_arr[0..5];
    println!("left: {:?}, right: {:?}", left, &test_arr[5..10]);

    let output: Vec<u32> = test_arr[0..5]
      .iter()
      .zip(test_arr[5..10].iter())
      .map(|(a, b)| a+b).collect();

    assert_eq!(output, vec![5, 7, 9, 11, 13]);
  }
}
