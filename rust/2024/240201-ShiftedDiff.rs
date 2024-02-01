// https://www.codewars.com/kata/5596f6e9529e9ab6fb000014

/**
 * fn shifted_diff(first: &str, second: &str) -> Option<usize> {
    (0..second.len()).map(|i| second[i..].to_string() + &second[..i]).position(|s| s == first)
}
 */

#[cfg(test)]
mod tests {
  fn shifted_diff(first: &str, second: &str) -> Option<usize> {
    let first_elem = first.chars().nth(0).unwrap();
    let find_result = second.chars().position(|x| x == first_elem);
    if find_result.is_none() {
      return None;
    }
    let index = find_result.unwrap();

    let mut rotated = String::from(&second[index..]);
    rotated.push_str(&second[0..index]);

    if rotated.as_str().eq(first) {
      return Some(index);
    }

    None
  }

  #[test]
  fn test_shifted_diff() {
    assert_eq!(shifted_diff("eecoff", "coffee"), Some(4));
    assert_eq!(shifted_diff("Moose", "moose"), None);
    assert_eq!(shifted_diff("isn't", "'tisn"), Some(2));
    assert_eq!(shifted_diff("Esham", "Esham"), Some(0));
  }

  #[test]
  fn test_str_slice() {
    let input: &str = "abcde";

    assert_eq!(input[0..2].eq("ab"), true);
  }

  #[test]
  fn test_str_rotation() {
    let input: &str = "abcde";
    let mut as_string = String::from(&input[2..]);

    as_string.push_str(&input[0..2]);

    assert_eq!(as_string, String::from("cdeab"));
  }

  #[test]
  fn find_element() {
    let input: &str = "abcde";

    assert_eq!(input.chars().nth(1).unwrap(), 'b');
    assert_eq!(input.chars().position(|x| x == 'b').unwrap(), 1);
  }
}
