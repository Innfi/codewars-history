// https://www.codewars.com/kata/5825792ada030e9601000782

/**
 * fn zip_with<F, T, S, R>(f: F, a: &[T], b: &[S]) -> Vec<R> 
where
    F: Fn(T, S) -> R,
    T: Copy,
    S: Copy
{
    a.iter().zip(b).map(|(&x, &y)| f(x, y)).collect()
}
 */

#[cfg(test)]
mod tests {
  fn add(a: i32, b: i32) -> i32 { a + b }
  fn zip_with<F, T, S, R>(f: F, a: &[T], b: &[S]) -> Vec<R> 
  where
      F: Fn(T, S) -> R,
      T: Copy,
      S: Copy
  {
    let len = match a.len() > b.len() {
      true => b.len(),
      _ => a.len()
    };
    (0..len).map(|i| f(a[i], b[i])).collect::<Vec<R>>()
  }

  #[test]
  fn returns_expected() {
    assert_eq!(&zip_with(add, &[0,1,2,3],     &[0,1,2,3]),     &[0,2,4,6]);
    assert_eq!(&zip_with(add, &[0,1,2,3],     &[0,1,2,3]),     &[0,2,4,6]);
  }
}
