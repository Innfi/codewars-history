//https://www.codewars.com/kata/56baeae7022c16dd7400086e

#[cfg(test)]
mod tests {
    use regex::Regex;

    fn dr() -> String {
        let dr0 = r#"/+1-541-754-3010 156 Alphand_St. <J Steeve>
        133, Green, Rd. <E Kustur> NY-56423 ;+1-541-914-3010;
        +1-541-984-3012 <P Reed> /PO Box 530; Pollocksville, NC-28573
        :+1-321-512-2222 <Paul Dive> Sequoia Alley PQ-67209
        +1-741-984-3090 <Peter Reedgrave> _Chicago
        :+1-921-333-2222 <Anna Stevens> Haramburu_Street AA-67209
        +1-111-544-8973 <Peter Pan> LA
        +1-921-512-2222 <Wilfrid Stevens> Wild Street AA-67209
        <Peter Gone> LA ?+1-121-544-8974 
        <R Steell> Quora Street AB-47209 +1-481-512-2222!
        <Arthur Clarke> San Antonio $+1-121-504-8974 TT-45120
        <Ray Chandler> Teliman Pk. !+1-681-512-2222! AB-47209,
        <Sophia Loren> +1-421-674-8974 Bern TP-46017
        <Peter O'Brien> High Street +1-908-512-2222; CC-47209
        <Anastasia> +48-421-674-8974 Via Quirinal Roma
        <P Salinger> Main Street, +1-098-512-2222, Denver
        <C Powel> *+19-421-674-8974 Chateau des Fosses Strasbourg F-68000
        <Bernard Deltheil> +1-498-512-2222; Mount Av.  Eldorado
        +1-099-500-8000 <Peter Crush> Labrador Bd.
        +1-931-512-4855 <William Saurin> Bison Street CQ-23071
        <P Salinge> Main Street, +1-098-512-2222, Denve
        /+5-541-754-3010 156 Alphandria_Street. <Jr Part>
        1333, Green, Road <F Fulgur> NW-46423 ;+6-541-914-3010!
        +5-541-984-3012 <Peter Reeves> /PO Box 5300; Albertville, SC-28573
        :+5-321-512-2222 <Paulo Divino> Boulder Alley ZQ-87209
        +3-741-984-3090 <F Flanaghan> _Chicago Av.
        :+3-921-333-2222 <Roland Scorsini> Bellevue_Street DA-67209
        +8-111-544-8973 <Laurence Pantow> SA
        +8-921-512-2222 <Raymond Stevenson> Joly Street EE-67209
        <John Freeland> Mantow ?+2-121-544-8974
        <Robert Mitch> Eleonore Street QB-87209 +2-481-512-2222?
        <Arthur Paternos> San Antonio $+7-121-504-8974 TT-45121
        <Ray Charles> Stevenson Pk. !+7-681-512-2222! CB-47209,
        <JP Gorce> +9-421-674-8974 New-Bern TP-16017
        <P McDon> Revolution Street +2-908-512-2222; PP-47209
        <Elizabeth Corber> +8-421-674-8974 Via Papa Roma
        <C Saborn> Main Street, +15-098-512-2222, Boulder
        <Colin Marshall> *+9-421-674-8974 Edinburgh UK
        <Bernard Povit> +3-498-512-2222; Hill Av.  Cameron
        +12-099-500-8000 <Pete Highman> Ontario Bd.
        +8-931-512-4855 <W Mount> Oxford Street CQ-23071
        <Donald Drinkaw> Moon Street, +3-098-512-2222, Peterville
        "#;
        return String::from(dr0);
    }

    #[test]
    fn test_filter() {
        let dir = &dr();

        let records: Vec<&str> = dir.as_str().split("\n").collect();

        let input_number: &str = "1-908-512-2222";
        let filter_result: Vec<&str> = records.iter()
            .filter(|x| x.contains(input_number))
            .cloned()
            .collect();
        assert_eq!(filter_result.is_empty(), false);
    }

    #[test]
    fn test_parsing() {
        let input: &str = "<Colin Marshall> *+9-421-674-8974 Edinburgh UK";

        let re_name = Regex::new(r"<(.+)>").unwrap();
        let result_name = re_name.captures(input).unwrap();
        assert_eq!(&result_name[1], "Colin Marshall");

        let first_filter = re_name.replace_all(input, "");
        let second_filter = first_filter.replace("+9-421-674-8974", "");

        assert_eq!(second_filter.trim_matches(' '), "* Edinburgh UK");
    }

    fn phone(dir: &str, num: &str) -> String {
        let re_filter = Regex::new(r"[;/_?*$!]").unwrap();
        let dir_filtered = re_filter.replace_all(dir, " ");
        let records: Vec<&str> = dir_filtered.split("\n").collect();

        let num_pattern = format!("+{}", num);

        let filter_result: Vec<&str> = records.iter()
            .filter(|x| x.contains(num_pattern.as_str()))
            .cloned()
            .collect();

        if filter_result.is_empty() {
            return format!("Error => Not found: {}", num);
        }
        if filter_result.len() > 1 {
            return format!("Error => Too many people: {}", num);
        }

        let re_name = Regex::new(r"<(.+)>").unwrap();
        let name = re_name.captures(filter_result[0]).unwrap();

        let first_filter = re_name.replace_all(filter_result[0], "");
        let addr = first_filter.replace(num_pattern.as_str(), "");

        let re_space = Regex::new(r"\s+").unwrap();
        let addr2 = re_space.replace_all(addr.as_str(), " ");

        format!("Phone => {}, Name => {}, Address => {}", 
            num,
            &name[1],
            addr2.trim_matches(' ')
        )
    }

    fn dotest(dir: &str, num: &str, exp: &str) -> () {
        println!("num:{}", num);
        let ans = phone(dir, num);
        println!("actual:\n{}", ans);
        println!("expect:\n{}", exp);
        println!("{}", ans == exp);
        assert_eq!(ans, exp);
        println!("{}", "-");
    }
    
    #[test]
    fn basis_tests() {
        let dir = &dr();
        dotest(dir, "48-421-674-8974", "Phone => 48-421-674-8974, Name => Anastasia, Address => Via Quirinal Roma");
        dotest(dir, "1-921-512-2222", "Phone => 1-921-512-2222, Name => Wilfrid Stevens, Address => Wild Street AA-67209");
        dotest(dir, "1-908-512-2222", "Phone => 1-908-512-2222, Name => Peter O'Brien, Address => High Street CC-47209");
        dotest(dir, "1-541-754-3010", "Phone => 1-541-754-3010, Name => J Steeve, Address => 156 Alphand St.");        
        dotest(dir, "1-098-512-2222", "Error => Too many people: 1-098-512-2222");
        dotest(dir, "5-555-555-5555", "Error => Not found: 5-555-555-5555");
        
    }
}
