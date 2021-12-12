//https://www.codewars.com/kata/587c0138110b20624e000253

/*
fn interpreter(tape: &str, data: &str) -> String {
    let mut instructions = tape.chars().cycle();
    let mut bits: Vec<char> = data.chars().collect();
    
    bits.into_iter()
        .map(|mut c| {
          while instructions.next().unwrap() == '1' {
              if c == '0' {
                  c = '1'
              } else {
                  c = '0'
              }
          }
          c
        }).collect()
}
*/

#[cfg(test)]
mod tests {
    fn flip(input: char) -> char {
        match input {
            '1' => '0',
            _ => '1'
        }
    }

    fn interpreter(tape: &str, data: &str) -> String {
        let tape_splitted: Vec<char> = tape.chars().collect();
        let mut data_splitted: Vec<char> = data.chars().collect();

        let mut tape_index = 0;
        let mut data_index = 0;

        while data_index < data.len() {
            if tape_splitted[tape_index] == '1' {
                data_splitted[data_index] = flip(data_splitted[data_index]);
            } else {
                data_index += 1;
            }

            tape_index += 1;
            if tape_index >= tape.len() {
                tape_index = 0;
            }
        }

        data_splitted.into_iter().collect()
    }

    #[test]
    fn basic_tests() {
        // flip every bit
        assert_eq!(interpreter("10", "1010"), "0101");
        // flip every other bit
        assert_eq!(interpreter("100", "1111111111"), "0101010101");
    }
}
