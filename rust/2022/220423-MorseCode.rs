//https://www.codewars.com/kata/54b72c16cd7f5154e9000457

/*
mod preloaded;
use preloaded::MORSE_CODE;
// MORSE_CODE is `HashMap<String, String>`. e.g. ".-" -> "A".

pub fn decode_bits(encoded: &str) -> String {
    // Trim excess zeros at the start and end
    let encoded = encoded.trim_matches('0');
    
    // Get the length of a time unit by finding the shortest sequence of zeros or ones,
    // this will represent a time unit of one which equals a dot
    let rate = {
        let rate_ones = encoded
            .split("0")
            .filter_map(|ones| (!ones.is_empty()).then(|| ones.len()))
            .min()
            .unwrap_or(usize::MAX);
        let rate_zeros = encoded
            .split("1")
            .filter_map(|zeros| (!zeros.is_empty()).then(|| zeros.len()))
            .min()
            .unwrap_or(usize::MAX);
        rate_zeros.min(rate_ones)
    };

    // Parse the encoded message
    encoded
        .chars() // Iterate through the characters
        .step_by(rate) // Only parse every n-th code
        .collect::<String>() // Collect it into a string
        // Begin converting from 1/0 to dot/dash
        .replace("111", "-") // Dash
        .replace("1", ".") // Dot
        .replace("0000000", "   ") // Word seperator
        .replace("000", " ") // Letter seperator
        .replace("0", "") // Dot/Dash seperator
}

pub fn decode_morse(encoded: &str) -> String {
    encoded
        .trim()
        .split("   ")
        .map(|word| {
            word.split(" ")
                .filter_map(|letter| MORSE_CODE.get(letter).map(|letter| letter.clone()))
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join(" ")
}
*/

#[cfg(test)]
mod tests {
    use std::str;

    #[test]
    fn test_split_by4() {
        let str_input: String = format!("11001100");

        let collected: Vec<&str> = str_input.as_bytes()
            .chunks(4).map(|x| {
                let symbol = str::from_utf8(x).unwrap();
                match symbol {
                    "1100" => ".",
                    "0000" => " ",
                    "1111" => "-",
                    _ => ""
                }
            })
            .collect::<Vec<&str>>();
        assert_eq!(collected.len(), 2);
    }

    pub fn decode_bits(encoded: &str) -> String {
        println!("encoded: {}", encoded);
        encoded
        .trim()
        .replace("11111100", "-")
        .replace("1100", ".")
        .replace("0000", " ")
        .replace("11", ".")
    }
    
    pub fn decode_morse(encoded: &str) -> String {
        encoded.replace("   ", " T ").split_ascii_whitespace().map(|x| {
            let output = match x {
                "T" => format!(" "),
                _ => MORSE_CODE.get(x).unwrap().to_owned()
            };
            output
        }).collect()
    }

    #[test]
    fn test_decode_bits() {
        let input: &str = "1100110011001100000011000000111111001100111111001111110000000000000011001111110011111100111111000000110011001111110000001111110011001100000011";

        assert_eq!(decode_bits(input), format!("···· · −·−−   ·−−− ··− −·· ·"));
    }

    #[test]
    fn test_str_replace() {
        let input: &str = "a   b";
        let output = input.replace("   ", " T ");
        assert_eq!(output, format!("a T b"));
    }

    // #[test]
    // fn examples() {
    //     assert_eq!(decode_morse(&decode_bits(input)), "HEY JUDE".to_string());
    // }
}
