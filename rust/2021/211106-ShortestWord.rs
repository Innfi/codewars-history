//https://www.codewars.com/kata/57cebe1dc6fdc20c57000ac9

/*
fn find_short(s: &str) -> usize {
  s.split_whitespace().map(str::len).min().unwrap()
}

fn find_short(s: &str) -> u32 {
  s.split_whitespace()
   .map(|word| word.len())
   .min()
   .unwrap_or(0) as u32
}
*/



#[cfg(test)]
mod tests {    
    #[test]
    fn test_str_split_whitespace() {
        let input: &str = "bitcoin take over the world maybe who knows perhaps";
        let shortest: &str = input.split_ascii_whitespace().reduce(|a: &str, b: &str| {
            match a.len() > b.len() {
                true => b,
                _ => a
            }
        }).unwrap();

        assert_eq!(shortest.len(), 3);
    }
    
    fn find_short(s: &str) -> u32 {
        s.split_ascii_whitespace().reduce(|a: &str, b: &str| {
            match a.len() > b.len() {
                true => b, 
                _ => a
            }
        }).unwrap().len() as u32
    }

    #[test]
    fn returns_expected() {
    assert_eq!(find_short("bitcoin take over the world maybe who knows perhaps"), 3);
    assert_eq!(find_short("turns out random test cases are easier than writing out basic ones"), 3);
    assert_eq!(find_short("lets talk about javascript the best language"), 3);
    assert_eq!(find_short("i want to travel the world writing code one day"), 1);
    assert_eq!(find_short("Lets all go on holiday somewhere very cold"), 2);
    assert_eq!(find_short("Let's travel abroad shall we"), 2);
    }
}
