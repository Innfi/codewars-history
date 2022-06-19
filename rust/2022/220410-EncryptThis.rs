//https://www.codewars.com/kata/5848565e273af816fb000449

#[cfg(test)]
mod tests {
    #[test]
    fn character_indexing() {
        let input: &str = "abcde";
        let last_index = input.len();
        let mut chars_array: Vec<char> = input.chars().collect();

        chars_array.swap(1, last_index-1);

        assert_eq!(chars_array[0], 'a');
        assert_eq!(chars_array.last().unwrap(), &'b');
    }

    #[test]
    fn char_to_digit() {
        let input = 'a';

        assert_eq!(format!("{}", (input as u32)), format!("97"));
    }

    #[test]
    fn split_whitespace() {
        let input: &str = "A wise old owl lived in an oak";
        let splitted: Vec<&str> = input.split_ascii_whitespace().collect();

        assert_eq!(splitted.len(), 8);
    }

    #[test]
    fn vec_pop() {
        let input = vec!['a', 'b', 'c'];
        let skipped: String = input.into_iter().skip(1).collect();

        assert_eq!(skipped.as_str(), "bc");

        let empty_input: Vec<char> = vec![];
        let empty_skipped: String = empty_input.into_iter().skip(1).collect();

        assert_eq!(format!("{}", empty_skipped), String::from(""));
    }

    fn encrypt_this(text: &str) -> String {
        text.split_ascii_whitespace().map(|x: &str| {
            let mut splitted: Vec<char> = x.chars().collect();
            let last_index = splitted.len()-1;
            let to_ascii_number: String = format!("{}", splitted[0] as u32);

            if splitted.len() == 1 { 
                return format!("{}", to_ascii_number);
            }

            splitted.swap(1, last_index);
            let skipped: String = splitted.into_iter().skip(1).collect();

            format!("{}{}", to_ascii_number, skipped)
        }).collect::<Vec<String>>().join(" ")
    }

    #[test]
    fn test_basic() {
        assert_eq!(encrypt_this(&"A"), "65".to_string());
        assert_eq!(encrypt_this(&"A wise old owl lived in an oak"), "65 119esi 111dl 111lw 108dvei 105n 97n 111ka".to_string());
        assert_eq!(encrypt_this(&"The more he saw the less he spoke"), "84eh 109ero 104e 115wa 116eh 108sse 104e 115eokp".to_string());
        assert_eq!(encrypt_this(&"The less he spoke the more he heard"), "84eh 108sse 104e 115eokp 116eh 109ero 104e 104dare".to_string());
        assert_eq!(encrypt_this(&"Why can we not all be like that wise old bird"), "87yh 99na 119e 110to 97ll 98e 108eki 116tah 119esi 111dl 98dri".to_string());
        assert_eq!(encrypt_this(&"Thank you Piotr for all your help"), "84kanh 121uo 80roti 102ro 97ll 121ruo 104ple".to_string());
    }
}
