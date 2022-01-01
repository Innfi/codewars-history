//https://www.codewars.com/kata/57eadb7ecd143f4c9c0000a3

#[cfg(test)]
mod tests {
    use std::str;

/*  best practice:
    fn abbrev_name(name: &str) -> String {
        name.split(' ')
        .map(|x| x.chars().nth(0).unwrap().to_string().to_uppercase())
        .collect::<Vec<_>>()
        .join(".")
    }
*/

    fn abbrev_name(name: &str) -> String {
        let split = name.split_whitespace().map(|s| String::from(s.chars().next().unwrap() ));

        split.reduce(|a, b| String::from(
            format!("{}.{}", a.to_uppercase(), b.to_uppercase()))).unwrap()
    }

    #[test]
    fn sample_tests() {
      assert_eq!(abbrev_name("Sam Harris"), "S.H");
      assert_eq!(abbrev_name("Patrick Feenan"), "P.F");
      assert_eq!(abbrev_name("Evan Cole"), "E.C");
      assert_eq!(abbrev_name("P Favuzzi"), "P.F");
      assert_eq!(abbrev_name("David Mendieta"), "D.M");
    }
}
