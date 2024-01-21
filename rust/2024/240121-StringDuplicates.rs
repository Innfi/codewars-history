// https://www.codewars.com/kata/59f08f89a5e129c543000069

/**
 * use itertools::Itertools;

  fn dup(ss: Vec<String>) -> Vec<String> {
      ss.iter().map(|s| s.chars().dedup().collect()).collect()
  }
 */
#[cfg(test)]
mod tests {
  fn dup(arry: Vec<String>) -> Vec<String> {
    arry.into_iter().map(|elem: String| {
      let mut output = String::new();
      elem.split("").into_iter().for_each(|x| {
        if output.is_empty() || !output[output.len()-1..].eq(x) {
          output.push_str(x);
        }
      });

      return output;
    }).collect()
  }

  #[test]
  fn string_test() {
    let input = String::from("abccd");
    let mut output = String::new();

    input.split("").into_iter().for_each(|x| {
      if output.is_empty() || !output[output.len()-1..].eq(x) {
        output.push_str(x);
      }
    });

    assert_eq!(output, String::from("abcd"));
  }

  #[test]
  fn returns_expected() {
    assert_eq!(1, 1);
  }
}

