//https://www.codewars.com/kata/52e88b39ffb6ac53a400022e

/*
fn int32_to_ip(int: u32) -> String {
    let bytes: [u8; 4] = int.to_be_bytes();
    format!("{}.{}.{}.{}", bytes[0], bytes[1], bytes[2], bytes[3])
}

fn int32_to_ip(int: u32) -> String {
    int.to_be_bytes().iter().map(u8::to_string).collect::<Vec<_>>().join(".")
}

*/

#[cfg(test)]
mod tests {
    use std::str;
    #[test]
    fn binary_to_integer() {
        let input: &str = "1101";
        let to_i32 = i32::from_str_radix(input, 2).expect("not a binary");

        assert_eq!(to_i32, 13);
    }

    #[test]
    fn integer_to_binary() {
        let input: u32 = 2149583361;
        let to_binary: String = format!("{:b}", input);
        
        assert_eq!(to_binary, String::from("10000000001000000000101000000001"));
    }

    #[test]
    fn substring_to_integer() {
        let input: String = String::from("10000000001000000000101000000001");
        let substring = input.get(0..8).unwrap();

        assert_eq!(substring, "10000000");
    }

    #[test]
    fn string_padding() {
        let input: u32 = 32;
        let to_binary: String = format!("{:032b}", input);
        
        assert_eq!(to_binary, String::from("00000000000000000000000000100000"));
    }

    fn int32_to_ip(input: u32) -> String {
        format!("{:032b}", input).as_bytes().chunks(8).map(str::from_utf8)
        .collect::<Result<Vec<&str>, _>>().unwrap().into_iter().map(|x| {
            format!("{}", u32::from_str_radix(x, 2).unwrap())
        }).reduce(|a, b| {
            format!("{}.{}", a, b)
        }).unwrap()
    }

    #[test]
    fn basic() {
        assert_eq!(int32_to_ip(2154959208), "128.114.17.104");
        assert_eq!(int32_to_ip(2149583361), "128.32.10.1");
        assert_eq!(int32_to_ip(0), "0.0.0.0");
    }
}
