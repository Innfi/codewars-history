//https://www.codewars.com/kata/57ebaa8f7b45ef590c00000c

/*
fn switcher(numbers: Vec<&str>) -> String {
    numbers.iter().map(|x| match x.parse::<u8>().unwrap() {
        29 => ' ',
        28 => '?',
        27 => '!',
        code => ('a' as u8 + (26 - code) as u8) as char
    }).collect()
}
*/

#[cfg(test)]
mod tests {
    
    #[test]
    fn string_to_number() {
        assert_eq!("24".parse::<u32>().unwrap(), 24);
    }

    #[test]
    fn vector_indexof() {
        let input = vec![ 'a', 'b', 'c', 'd' ];
        assert_eq!(input[2], 'c');
    }
    
    fn switcher(numbers: Vec<&str>) -> String {
        const input: Vec<char> = vec![ '|', 
            'z', 'y', 'x', 'w', 'v', 
            'u', 't', 's', 'r', 'q', 
            'p', 'o', 'n', 'm', 'l', 
            'k', 'j', 'i', 'h', 'g', 
            'f', 'e', 'd', 'c', 'b', 
            'a', '!', '?', ' ' ];

        numbers.into_iter().map(|number: &str| {
            input[number.parse::<usize>().unwrap()]
        }).collect()
    }

    #[test]
    fn example_tests() {
        assert_eq!(switcher(vec!["24", "12", "23", "22", "4", "26", "9", "8"]), "codewars");
        assert_eq!(switcher(vec!["25","7","8","4","14","23","8","25","23","29","16","16","4"]), "btswmdsbd kkw"); 
        assert_eq!(switcher(vec!["4", "24"]), "wc");
    }
}
