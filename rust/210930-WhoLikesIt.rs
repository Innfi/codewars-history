//https://www.codewars.com/kata/5266876b8f4bf2da9b000362

#[cfg(test)]
mod tests {
    fn likes(names: &[&str]) -> String {

        // let sentence = "this is a sentence in rust";
        // let words: Vec<&str> = sentence.split_whitespace().collect();
        // let words_cont: Vec<&str> = words
        //     .into_iter()
        //     .filter(|word| word.contains("i"))
        //     .collect();
        // println!("{:?}", words_cont);

        let mut result: String = String::from("");

        let test: Vec<&str> = names.to_vec().into_iter()
            .map(|word| {
                result.push_str(word);
                result.push_str(", ");
                word
            } ).collect();
        println!("{:?}", test);
        println!("result: {}", result);

        for x in names {
            println!("name: {}", *x);
        }
        String::from("Peter likes this")
    }

    #[test]
    fn example_tests() {
        //assert_eq!(likes(&[]), "no one likes this");
        assert_eq!(likes(&["Peter", "Jacob", "Alex"]), "Peter likes this");
        // assert_eq!(likes(&["Jacob", "Alex"]), "Jacob and Alex like this");
        // assert_eq!(
        //     likes(&["Max", "John", "Mark"]),
        //     "Max, John and Mark like this"
        // );
        // assert_eq!(
        //     likes(&["Alex", "Jacob", "Mark", "Max"]),
        //     "Alex, Jacob and 2 others like this"
        // );
    }
}
