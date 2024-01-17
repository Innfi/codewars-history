// https://www.codewars.com/kata/525f50e3b73515a6db000b83

#[cfg(test)]
mod tests {
  fn create_phone_number(numbers: &[u8]) -> String {
  let first = numbers[0..3].into_iter().map(|x| x.to_string()).collect::<String>();
  let middle = numbers[3..6].into_iter().map(|x| x.to_string()).collect::<String>();
  let last = numbers[6..].into_iter().map(|x| x.to_string()).collect::<String>();
    
  format!("({}) {}-{}", first, middle, last)
}

  #[test]
  fn returns_expected() {
    assert_eq!(1, 1);
  }

  #[test]
  fn test_array_slice() {
    let input: &[u8] = &[1, 2, 3, 4, 5];

    let sliced: &[u8] = &input[0..3];

    assert_eq!(sliced, &[1,2,3]);
  }

  #[test]
  fn test_array_join() {
    let input: &[u8] = &[1, 2, 3, 4, 5];

    let string_vec: Vec<String> = input.into_iter().map(|x| x.to_string()).collect::<Vec<String>>();

    assert_eq!(string_vec.join(""), String::from("12345"));

    let subset = input[0..3].into_iter().map(|x| x.to_string()).collect::<String>();
    assert_eq!(subset, String::from("123"));

    let remain = input[3..].into_iter().map(|x| x.to_string()).collect::<String>();
    assert_eq!(remain, String::from("45"));
  }
}

