//https://www.codewars.com/kata/53af2b8861023f1d88000832

/*
fn are_you_playing_banjo(name: &str) -> String {
    match &name[0..1] {
        "R" | "r" => format!("{} plays banjo", name),
        _ => format!("{} does not play banjo", name)
    }
}
*/

#[cfg(test)]
mod tests {
    fn are_you_playing_banjo(name: &str) -> String {
        match name.chars().nth(0) {
            Some('R') | Some('r') => format!("{} plays banjo", name),
            _ => format!("{} does not play banjo", name)
        }
    }

    #[test]
    fn test_are_you_playing_banjo() {
        assert_eq!(are_you_playing_banjo("Martin"), "Martin does not play banjo");
        assert_eq!(are_you_playing_banjo("Rikke"), "Rikke plays banjo");
        assert_eq!(are_you_playing_banjo("bravo"), "bravo does not play banjo");
        assert_eq!(are_you_playing_banjo("rolf"), "rolf plays banjo");
    }
}
