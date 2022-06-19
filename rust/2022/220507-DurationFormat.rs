//https://www.codewars.com/kata/52742f58faf5485cae000b9a

/*
fn format_duration(seconds: u64) -> String {
    let result = [
        ("year", 31536000, 100000),
        ("day", 86400, 365),
        ("hour", 3600, 24),
        ("minute", 60, 60),
        ("second", 1, 60),
    ].iter()
    .map(|(unit, duration, modulo)| (seconds / duration % modulo, unit))
    .filter_map(|(duration, unit)| match duration {
        _ if duration == 1 => Some(format!("{} {}", duration, unit)),
        _ if duration != 0 => Some(format!("{} {}s", duration, unit)),
        _ => None,
    }).collect::<Vec<String>>();

    match result.len() {
        0 => String::from("now"),
        1 => result.join(""),
        _ => result
            .split_last()
            .map(|(r, l)| l.join(", ") + " and " + r)
            .unwrap(),
    }
}
*/

#[cfg(test)]
mod tests {
    #[test]
    fn get_remain() {
        let input = 1234;

        assert_eq!(input / 86400, 0);
    }

    fn format_timestamps(
        seconds: u64,
        divisor: u64,
        singular: &str,
        plural: &str,
        output_vec: &mut Vec<String>
    ) -> u64 {
        let remain: u64 = seconds / divisor;
        let out_format = match remain {
            1 => format!("1 {}", singular),
            _ => format!("{} {}", remain, plural),
        };

        if remain > 0 { output_vec.push(out_format.clone()); }

        seconds - (remain*divisor)
    }

    fn format_duration(seconds: u64) -> String {
        if seconds == 0 { return format!("now"); }

        let mut out_vec: Vec<String> = Vec::new();

        let year_remain = format_timestamps(
            seconds, 86400*365, "year", "years", &mut out_vec);
        let day_remain = format_timestamps(
            year_remain, 86400, "day", "days", &mut out_vec);
        let hour_remain = format_timestamps(
            day_remain, 3600, "hour", "hours", &mut out_vec);
        let min_remain = format_timestamps(
            hour_remain, 60, "minute", "minutes", &mut out_vec);
        format_timestamps(min_remain, 1, "second", "seconds", &mut out_vec);

        let mut output_string = format!("");
        let mut count = 0;
        while !out_vec.is_empty() {
            let output = out_vec.pop().unwrap();
            match count {
                0 => output_string = format!("{}", output),
                1 => output_string = format!("{} and {}", output, output_string),
                _ => output_string = format!("{}, {}", output, output_string),
            };

            count += 1;
        }

        output_string
    }

    #[test]
    fn test_basic() {
        assert_eq!(format_duration(1), "1 second");
        assert_eq!(format_duration(62), "1 minute and 2 seconds");
        assert_eq!(format_duration(120), "2 minutes");
        assert_eq!(format_duration(3600), "1 hour");
        assert_eq!(format_duration(3662), "1 hour, 1 minute and 2 seconds");
        assert_eq!(format_duration(86400*365*3 + 86400*10 + 3600*15),
            "3 years, 10 days and 15 hours");
    }
}
