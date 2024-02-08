// https://www.codewars.com/kata/515decfd9dcfc23bb6000006

#[cfg(test)]
mod tests {
  fn is_valid_ip(ip: &str) -> bool {
    let splitted = ip.split('.');
    if splitted.clone().count() != 4 {
        return false;
    }
    
   for x in splitted {
      if x.len() > 1 && x.chars().nth(0).unwrap() == '0' {
        return false;
      }

      let parse_result = x.parse::<u32>();
      if parse_result.is_err() {
        return false;
      }

      if parse_result.unwrap() > 255 {
        return false;
      }
    };
    
    true
  }

  #[test]
  fn returns_expected() {
    // let input: &str = "1.2.3.04";
    let input: &str = "0.0.0.0";

    let splitted = input.split('.');
    assert_eq!(splitted.clone().count(), 4);
    let mut is_valid = true;

    for (_, x) in splitted.enumerate() {
      if x.len() > 1 && x.chars().nth(0).unwrap() == '0' {
        is_valid = false;
        break;
      }

      let parse_result = x.parse::<u32>();
      if parse_result.is_err() {
        is_valid = false;
        break;
      }

      let as_number: u32 = parse_result.unwrap();
      if as_number < 1 || as_number >= 255 {
        is_valid = false;
        break;
      }
    }

    assert_eq!(is_valid, false);
  }
}