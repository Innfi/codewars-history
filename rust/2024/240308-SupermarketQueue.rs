// https://www.codewars.com/kata/57b06f90e298a7b53d000a86

#[cfg(test)]
mod tests {
  fn queue_time(customers: &[u32], n: u32) -> u32 {
    todo!()
  }

  #[test]
  fn returns_expected() {
    assert_eq!(queue_time([5,3,4], 1), 12);
    assert_eq!(queue_time([10.,2,3,3], 2), 10);
    assert_eq!(queue_time([2,3,10], 2), 12);
  }
}
