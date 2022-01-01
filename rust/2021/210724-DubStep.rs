#[cfg(test)]
mod tests {
    use regex::Regex;
    fn song_decoder(song: &str) -> String {
        //String::from(song.replace("WUB", " ").replace("  ", " ").trim())
        let re = Regex::new(r"(WUB)+").unwrap();

        String::from( re.replace_all(song, " ").trim() )
    }

    #[test]
    fn test_remove_maches() {
        assert_eq!("dubabcdub".replace("dub", ""), "abc");
    }

    #[test]
    fn test_regex() {
        let re = Regex::new(r"(dub)+").unwrap();

        assert_eq!(re.replace_all("dubabcdubdub", " "), " abc ");
    }

    #[test]
    fn returns_expected() {
        assert_eq!(song_decoder("WUBAWUBWUBC"), "A C");
        assert_eq!(song_decoder("AWUBWUBWUBBWUBWUBWUBC"), "A B C");
        assert_eq!(song_decoder("WUBAWUBBWUBCWUB"), "A B C");
        assert_eq!(song_decoder("AWUBBWUBC"), "A B C");
    }
}