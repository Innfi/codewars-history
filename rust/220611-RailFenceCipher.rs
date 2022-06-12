//https://www.codewars.com/kata/58c5577d61aefcf3ff000081

#[cfg(test)]
mod tests {
    fn pendulum(len: usize, bucket_count: usize) -> Vec<(usize, i32)> {
        let mut output: Vec<(usize, i32)> = Vec::new();
        let mut current_bucket: i32 = 0;
        let mut direction: i32 = 1;
        let last = (bucket_count -1) as i32;

        if bucket_count == 1 {
            return (0..len).into_iter().map(|x| (x, 0)).collect();
        }

        for i in 0..len {
            output.push((i, current_bucket));

            if current_bucket == 0 {
                direction = 1;
            }
            else if current_bucket == last {
                direction = -1;
            }
            
            current_bucket += direction;
        }

        output
    }

    #[test]
    fn test_pendulum_single_bucket() {
        let len = 10;
        let bucket_count = 1;

        let output = pendulum(len, bucket_count);
        assert_eq!(output.into_iter().all(|(_, position)| {
            position == 0
        }), true);
    }

    #[test]
    fn test_pendulum_double_bucket() {
        let len = 10;
        let bucket_count = 2;

        let output = pendulum(len, bucket_count);

        for x in 0..output.len() {
            let element = &output[x];
            if x % 2 == 0 { assert_eq!(element.1, 0); }
            else { assert_eq!(element.1, 1); }
        }
    }

    #[test]
    fn test_pendulum_triple_bucket() {
        let len = 5;
        let bucket_count = 3;

        let output = pendulum(len, bucket_count);

        assert_eq!(output, vec![(0, 0), (1, 1), (2, 2), (3, 1), (4, 0)]);
    }

    fn encode_rail_fence_cipher(text: &str, num_rails: usize) -> String {
        let pendulum_indexes = pendulum(text.len(), num_rails);
        let input: Vec<char> = text.chars().collect();

        (0..num_rails).map(|i| {
            pendulum_indexes
                .iter()
                .filter(|x| x.1 == i as i32).map(|(pi, _)| {
                    input[*pi]
                }).collect()
        }).collect::<Vec<String>>().join("")
    }

    #[test]
    fn test_decoding() {
        let input: &str = "WECRLTEERDSOEEFEAOCAIVDEN";
        let bucket_count = 3;
        let pendulum_indexes = pendulum(input.len(), bucket_count);

        let mut start = 0;
        let mut output_vec: Vec<(usize, char)> = Vec::new();

        (0..bucket_count).for_each(|i| {
            let filtered = pendulum_indexes.iter().filter(|x| x.1 == i as i32)
                .map(|(pi, bucket)| (*pi, *bucket))
                .collect::<Vec<(usize, i32)>>();
            let end = start + filtered.len();
            let substring = input.get(start..end).unwrap();

            substring.chars().enumerate().for_each(|(si, c)| {
                output_vec.push((filtered[si].0, c));
            });

            start = end;
        });

        assert_eq!(output_vec.len(), input.len());

        output_vec.sort_by(|a, b| a.0.cmp(&b.0));

        let out_string: String = output_vec.into_iter().map(|x| x.1).collect();
        assert_eq!(out_string, String::from("WEAREDISCOVEREDFLEEATONCE"));
    }

    fn decode_rail_fence_cipher(text: &str, num_rails: usize) -> String {
        let pendulum_indexes = pendulum(text.len(), num_rails);

        let mut start = 0;
        let mut output_vec: Vec<(usize, char)> = Vec::new();

        (0..num_rails).for_each(|rail_index| {
            let filtered_indexes = pendulum_indexes.iter()
                .filter(|x| x.1 == rail_index as i32)
                .map(|(pi, bucket)| (*pi, *bucket))
                .collect::<Vec<(usize, i32)>>();
            let end = start + filtered_indexes.len();
            
            let substring = text.get(start..end).unwrap();
            substring.chars().enumerate().for_each(|(si, c)| {
                output_vec.push((filtered_indexes[si].0, c));
            });

            start = end;
        });

        output_vec.sort_by(|a, b| a.0.cmp(&b.0));

        output_vec.into_iter().map(|x| x.1).collect()
    }

    #[test]
    fn basic_tests() {
        assert_eq!(
            encode_rail_fence_cipher("WEAREDISCOVEREDFLEEATONCE", 3),
            "WECRLTEERDSOEEFEAOCAIVDEN"
        );
        assert_eq!(
            decode_rail_fence_cipher("WECRLTEERDSOEEFEAOCAIVDEN", 3),
            "WEAREDISCOVEREDFLEEATONCE"
        );
        assert_eq!(
            encode_rail_fence_cipher("Hello, World!", 3),
            "Hoo!el,Wrdl l"
        );
        assert_eq!(
            decode_rail_fence_cipher("Hoo!el,Wrdl l", 3),
            "Hello, World!"
        );
    }
}
