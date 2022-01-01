//https://www.codewars.com/kata/5a53a17bfd56cb9c14000003

/*
fn disarium_number(n: u32) -> String {
    if n.to_string()
        .chars()
        .filter_map(|ch| ch.to_digit(10))
        .enumerate()
        .fold(0_u32, |acc, digit| acc + digit.1.pow(digit.0 as u32 + 1))
        == n
    {
        "Disarium !!".to_string()
    } else {
        "Not !!".to_string()
    }
}
*/
#[cfg(test)]
mod tests {
    #[test]
    fn test_number_split() {
        let input: u32 = 15243;

        let input_str: String = input.to_string();
        let out: u32 = input_str.chars().map(|x: char| {
            let trying: u32 = x.to_string().parse::<u32>().unwrap();

            return trying;
        }).sum();

        assert_eq!(out, 15);
    }

    fn disarium_number(n: u32) -> String {
        let mut pow = 0;
        let result: u32 = n.to_string().chars().map(|x: char| {
            pow += 1;
            x.to_string().parse::<u32>().unwrap().pow(pow)
        }).sum();

        match result == n {
            true => String::from("Disarium !!"), 
            _ => String::from("Not !!")
        }
    }

    #[test]
    fn basic() {
        assert_eq!(disarium_number(89),"Disarium !!");
        assert_eq!(disarium_number(564),"Not !!");
        assert_eq!(disarium_number(1024),"Not !!");
        assert_eq!(disarium_number(135),"Disarium !!");
        assert_eq!(disarium_number(136586),"Not !!");        
    }
}
