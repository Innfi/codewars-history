// https://www.codewars.com/kata/554ca54ffa7d91b236000023

#[cfg(test)]
mod tests {
  use std::{cmp::Ordering, collections::HashMap};

  fn delete_nth(lst: &[u8], n: usize) -> Vec<u8> {
    let mut dict: HashMap<u8, usize> = HashMap::new();
    let mut result: Vec<u8> = Vec::new();

    for elem in lst {
      match dict.get(elem) {
        None => {
          dict.insert(*elem, 1);
          result.push(*elem);
        },
        Some(v) => {
          if *v < n {
            dict.insert(*elem, v+1);
            result.push(*elem);
          }
        },
      }
    }

    result
  }

  #[test]
  fn test_delete_nth() {
    assert_eq!(delete_nth(&[20,37,20,21], 1), vec![20,37,21]);
    assert_eq!(delete_nth(&[1,1,3,3,7,2,2,2,2], 3), vec![1, 1, 3, 3, 7, 2, 2, 2]);
  }

  #[test]
  fn test_hashmap() {
    let mut init_map: HashMap<u32, u32> = HashMap::new();
    init_map.insert(1, 5);
    init_map.insert(3, 4);
    init_map.insert(5, 10);

    let get_result = init_map.get_key_value(&1);
    assert_eq!(get_result.is_some(), true);

    let invalid_result = init_map.get_key_value(&55);
    assert_eq!(invalid_result.is_some(), false);

    init_map.insert(1, 11);
    let result = init_map.get_key_value(&1).unwrap();
    assert_eq!(result.0.cmp(&1), Ordering::Equal);
    assert_eq!(result.1.cmp(&11), Ordering::Equal);
  }
}
