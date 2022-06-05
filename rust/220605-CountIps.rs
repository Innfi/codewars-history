//https://www.codewars.com/kata/526989a41034285187000de4

#[cfg(test)]
mod tests {
    #[test]
    fn parse_ip() {
        let input: &str = "10.1.2.3";

        let to_string = String::from(input);
        let output: Vec<&str> = to_string.split('.').collect::<Vec<&str>>();
        
        assert_eq!(output.len(), 4);
        assert_eq!(output[0], "10");
    }

    #[test]
    fn parse_int() {
        let input: &str = "10";

        let output: u32 = input.parse::<u32>().unwrap();

        assert_eq!(output, 10);
    }

    #[test]
    fn test_powi() {
        let input: u32 = 256;

        assert_eq!(input.pow(2), 65536);
    }

    #[test]
    fn ip_to_digits() {
        let input: &str = "0.1.2.3";
        let to_string = String::from(input);

        let output: u32 = to_string.split('.').collect::<Vec<&str>>()
            .into_iter().rev().enumerate().map(|(index, x)| {
                let output = x.parse::<u32>().unwrap();

                if index == 0 { return output; }
                else {
                    let multi = (256 as u32).pow((index) as u32);
                    return output * multi;
                }
            }).sum();
        assert_eq!(output, 65536+(256*2)+3);
    }

    fn to_digits(input: String) -> u32 {
        input.split('.').rev().enumerate().map(|(i, x)| {
            let output = x.parse::<u32>().unwrap();
            let multi = if i == 0 { 1 } else { (256 as u32).pow(i as u32) };

            output * multi
        }).sum()
    }

    fn ips_between(start: &str, end: &str) -> u32 {
        to_digits(String::from(end)) - to_digits(String::from(start))
    }

    #[test]
    fn basic() {
        assert_eq!(ips_between("10.0.0.0", "10.0.0.50"), 50);
        assert_eq!(ips_between("20.0.0.10", "20.0.1.0"), 246);
    }
}
