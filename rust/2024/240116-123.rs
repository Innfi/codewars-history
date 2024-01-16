// https://www.codewars.com/kata/54bf85e3d5b56c7a05000cf9

#[cfg(test)]
mod tests {
  fn number(lines: &[&str]) -> Vec<String> {
    lines.into_iter().enumerate().map(|(i, x)| {
      format!("{}: {}", i+1, x)
    }).collect()
  }

  #[test]
  fn returns_expected() {
    let result = number(&["a", "b", "c"]);
    let expected: Vec<String> = vec![
      "1: a".into(), 
      "2: b".into(), 
      "3: c".into()
    ];

    assert_eq!(result, expected);
  }
}

