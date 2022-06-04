//https://www.codewars.com/kata/551f23362ff852e2ab000037

/*
fn longest_slide_down(pyramid: &[Vec<u16>]) -> u16 {
    pyramid .to_owned()
        .into_iter()
        .rev()
        .reduce(|floor,ceil|
                ceil.iter()
                    .enumerate()
                    .map(|(i,&n)|  std::cmp::max(floor[i], floor[i+1]) + n )
                    .collect::<Vec<u16>>()
            )
        .unwrap() [0]
}
*/

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    #[test]
    fn test_indexing() {
        let input = vec![
               vec![ 3],
              vec![7, 4],
             vec![ 2, 4, 6],
            vec![8, 5, 9, 3]
        ];

        let mut deque: VecDeque<(usize, u16)> = VecDeque::new();

        input.into_iter().enumerate().for_each(|(index, layer)| {
            if index == 0 {
                deque.push_back((0, layer[0]));
                return;
            }

            for _ in 0..index {
                let start = deque.pop_front().unwrap();

                deque.push_back((start.0, start.1+layer[start.0]));
                deque.push_back((start.0+1, start.1+layer[start.0+1]));
            }
        });

        let mut vec: Vec<u16> = deque.into_iter().map(|x| x.1).collect();
        vec.sort_by(|a, b| b.cmp(a));

        assert_eq!(vec[0], 23);
    }

    fn longest_slide_down(pyramid: &[Vec<u16>]) -> u16 {
        let mut deque: VecDeque<(usize, u16)> = VecDeque::new();
        
        for index in 0..pyramid.len() {
            let layer = &pyramid[index];
            if index == 0 {
                deque.push_back((0, layer[0]));
                continue;
            }

            for _ in 0..deque.len() {
                let start = deque.pop_front().unwrap();

                deque.push_back((start.0, start.1+layer[start.0]));
                deque.push_back((start.0+1, start.1+layer[start.0+1]));

            }
        }

        let output = deque.into_iter().reduce(|a, b| {
            if a.1 > b.1 { return a; }
            else { return b; }
        }).unwrap();

        output.1

        // let mut vec:Vec<u16> = deque.into_iter().map(|x| x.1).collect();
        // vec.sort_by(|a, b| b.cmp(a));
        
        // vec[0]
    }

    #[test]
    fn test_small() {
        let small = vec![
            vec![3],
            vec![7, 4],
            vec![2, 4, 6],
            vec![8, 5, 9, 3]
        ];
        assert_eq!(longest_slide_down(&small), 23, "It should work for small pyramids");
    }

    #[test]
    fn test_medium() {
        let medium = vec![
                                  vec![75],
                                 vec![95, 64],
                               vec![17, 47, 82],
                             vec![18, 35, 87, 10],
                           vec![20,  4, 82, 47, 65],
                          vec![19,  1, 23, 75,  3, 34],
                         vec![88,  2, 77, 73,  7, 63, 67],
                        vec![99, 65,  4, 28,  6, 16, 70, 92],
                      vec![41, 41, 26, 56, 83, 40, 80, 70, 33],
                    vec![41, 48, 72, 33, 47, 32, 37, 16, 94, 29],
                   vec![53, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14],
                 vec![70, 11, 33, 28, 77, 73, 17, 78, 39, 68, 17, 57],
                vec![91, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48],
              vec![63, 66,  4, 68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31],
            vec![ 4, 62, 98, 27, 23,  9, 70, 98, 73, 93, 38, 53, 60,  4, 23]
        ];
        assert_eq!(longest_slide_down(&medium), 1074, "It should work for medium pyramids");
    }
}
