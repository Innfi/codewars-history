//https://www.codewars.com/kata/523a86aa4230ebb5420001e1

/*
use itertools::Itertools;

fn anagrams(word: &str, words: &[String]) -> Vec<String> {
    let cs = word.chars().sorted().collect_vec();
    words.iter().filter(|s| s.chars().sorted().collect_vec() == cs).cloned().collect()
}
*/

#[cfg(test)]
mod tests {
    
    #[test]
    fn sort_and_compare() {
        let input = &["aabb", "abcd", "bbaa", "dada"];
        let words: Vec<String> = input.iter().map(|w| w.to_string()).collect();

        assert_eq!(words.len(), 4);
        words.into_iter().for_each(|x| {
            let mut splitted: Vec<char> = x.chars().collect();
            splitted.sort_by(|a, b| a.cmp(b));

            let output: String = splitted.into_iter().collect();
            println!("output: {}", output);
        });
    }

    fn anagrams(word: &str, words: &[String]) -> Vec<String> {
        let mut output_vec: Vec<String> = Vec::new();

        let mut splitted_word: Vec<char> = word.chars().collect();
        splitted_word.sort_by(|a, b| a.cmp(b));
        let target_word: String = splitted_word.into_iter().collect();

        for x in words {
            let mut splitted: Vec<char> = x.chars().collect();
            splitted.sort_by(|a, b| a.cmp(b));

            let output: String = splitted.into_iter().collect();
            if output == target_word {
                output_vec.push(x.clone());
            }
        }

        output_vec
    }

    #[test]
    fn sample_tests() {
        do_test("abba", &["aabb", "abcd", "bbaa", "dada"], &["aabb", "bbaa"]);
        
        do_test(
            "racer",
            &["crazer", "carer", "racar", "caers", "racer"],
            &["carer", "racer"],
        );
    }

    fn do_test(word: &str, words: &[&str], exp: &[&str]) {
        let words: Vec<String> = words.iter().map(|w| w.to_string()).collect();
        let expected: Vec<String> = exp.iter().map(|w| w.to_string()).collect();
        let got = anagrams(word, &words);
        assert_eq!(
            got, expected,
            "Failed with word: \"{}\"\nwords: {:#?}",
            word, words
        );
    }
}