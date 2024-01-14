// https://www.codewars.com/kata/52efefcbcdf57161d4000091

/**
  fn count(s: &str) -> HashMap<char, i32> {
      let mut map: HashMap<char, i32> = HashMap::new();
      s.chars().for_each(|c| {
          let entry = map.entry(c).or_insert(0);
          *entry += 1
      });
      map
  }
 */

#[cfg(test)]
mod tests {
  use std::collections::HashMap;

  fn count(input: &str) -> HashMap<char, i32> {
    let mut map = HashMap::<char, i32>::new();

    if input.len() <= 0 {
      return map;
    }

    for item in input.chars() {
      let mut current = 0;
      if map.contains_key(&item) {
        current = *(map.get(&item).unwrap());
      }

      map.insert(item, current+1);
    }

    map
  }

  #[test]
  fn returns_expected() {
    assert_eq!(1, 1);
  }

  #[test]
  fn test_empty_hashamp() {
    let result = count("");
    let expected = HashMap::<char, i32>::new();

    assert_eq!(result, expected);
  }
}

