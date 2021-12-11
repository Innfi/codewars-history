//https://www.codewars.com/kata/57f625992f4d53c24200070e

/*
fn bingo<S: AsRef<str>>(ticket: &[(S, u8)], win: usize) -> &'static str {
    let score = ticket.iter().filter(|(s, n)| s.as_ref().as_bytes().contains(n)).count();
    if score >= win { "Winner!" } else { "Loser!" }
}
*/

#[cfg(test)]
mod tests {
    #[test]
    fn array_foreach() {
        let input = &[("ABC", 65), ("HGR", 74)];

        input.iter().for_each(|x| {
            //let test_string: &str = x.0;
            let test_number: u8 = x.1;

            let test_letter: char = test_number as char;
            println!("test_letter: {}", test_letter);

            // let find_result = 
            //     test_string.chars().find(|x| *x == test_letter).unwrap();
            
        });
    }

    #[test]
    fn array_findfail() {
        let input: &str = "abcd";
        assert_eq!(input.chars().find(|x| *x == 'c').unwrap(), 'c');
        assert_eq!(input.chars().find(|x| *x == 'D'), None);
    }

    fn bingo<S: AsRef<str>>(ticket: &[(S, u8)], win: usize) -> &'static str {
        let result: usize = ticket.iter().map(|(token, num)| {
            match token.as_ref().chars().find(|x| *x == *num as char) {
                None => 0,
                _ => 1
            }
        }).sum();
        
        match result >= win {
            true => "Winner!",
            _ => "Loser!"
        }
    }

    #[test]
    fn basic() {
        assert_eq!(bingo(&[("ABC", 65), ("HGR", 74), ("BYHT", 74)], 2), "Loser!");
        assert_eq!(bingo(&[("ABC", 65), ("HGR", 74), ("BYHT", 74)], 1), "Winner!");
        assert_eq!(bingo(&[("HGTYRE", 74), ("BE", 66), ("JKTY", 74)], 3), "Loser!");
    }
}
