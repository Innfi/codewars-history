//https://www.codewars.com/kata/57eb8fcdf670e99d9b000272

/*
fn high(input: &str) -> &str {
    input.split_whitespace().min_by_key(|word|
        - word.chars().map(|c| (c as i32) - 96).sum::<i32>()
    ).unwrap_or("")
}
*/

#[cfg(test)]
mod tests {
    #[test]
    fn test_map() {
        let input: &str = "man i need a taxi up to ubud";

        let output: (&str, u32) = input.split_ascii_whitespace()
            .map(|x: &str| {
                let sum: u32 = x.chars().map(|letter| {
                    (letter as u32) - 96
                }).sum();

                (x, sum)
            }).reduce(|a: (&str, u32), b: (&str, u32)| {
                if a.1 > b.1 { return a; }
                else { return b; }
            }).unwrap();

        assert_eq!(output.0, "taxi");
        assert_eq!(output.1, 54);
    }

    fn high(input: &str) -> &str {
        input.split_ascii_whitespace()
            .map(|x: &str| {
                (x, x.chars().map(|letter| (letter as u32) - 96).sum())
            }).reduce(|a: (&str, u32), b: (&str, u32)| {
                if a.1 == b.1 { return a; }
                else if a.1 > b.1 { return a; }
                else { return b; }
            }).unwrap().0
    }

    #[test]
    fn test_basic() {
        assert_eq!(high("man i need a taxi up to ubud"), "taxi");               
        assert_eq!(high("what time are we climbing up the volcano"), "volcano");
        assert_eq!(high("take me to semynak"), "semynak");                      
        assert_eq!(high("massage yes massage yes massage"), "massage");         
        assert_eq!(high("take two bintang and a dance please"), "bintang"); 
        assert_eq!(high("aa b"), "aa");         
        assert_eq!(high("b aa"), "b");     
        assert_eq!(high("bb d"), "bb");                            
        assert_eq!(high("d bb"), "d"); 
        assert_eq!(high("aaa b"), "aaa");                                     
    }
}
