//https://www.codewars.com/kata/534e01fbbb17187c7e0000c6

#[cfg(test)]
mod tests {
    #[test]
    fn create_array2d() {
        let array_2d: Vec<Vec<i8>> = vec![ vec![0; 3]; 3 ];

        array_2d.into_iter().for_each(|x| {
            assert_eq!(x.into_iter().any(|element| element == 0), true);
        });
    }

    fn array_right(array_2d: &mut Vec<Vec<i8>>, x: usize, y: usize) {
        for i in 0..x {
            array_2d[i][y-1] = 1;
        }
    }

    #[test]
    fn traverse_array2d() {
        let mut array_2d: Vec<Vec<i8>> = vec![ vec![0; 3]; 3 ];
        array_right(&mut array_2d, 3, 3);
        let first = array_2d.first().cloned().unwrap();

        assert_eq!(first.into_iter().any(|x| x == 1), true);
    }

    // fn spiralize(size: usize) -> Vec<Vec<i8>> {
    //     unimplemented!();
    // }

    // #[test]
    // fn test5() {
    //     assert_eq!(
    //         spiralize(5),
    //         [
    //             [1, 1, 1, 1, 1],
    //             [0, 0, 0, 0, 1],
    //             [1, 1, 1, 0, 1],
    //             [1, 0, 0, 0, 1],
    //             [1, 1, 1, 1, 1],
    //         ],
    //     );
    // }

    // #[test]
    // fn test8() {
    //     assert_eq!(
    //         spiralize(8),
    //         [
    //             [1, 1, 1, 1, 1, 1, 1, 1],
    //             [0, 0, 0, 0, 0, 0, 0, 1],
    //             [1, 1, 1, 1, 1, 1, 0, 1],
    //             [1, 0, 0, 0, 0, 1, 0, 1],
    //             [1, 0, 1, 0, 0, 1, 0, 1],
    //             [1, 0, 1, 1, 1, 1, 0, 1],
    //             [1, 0, 0, 0, 0, 0, 0, 1],
    //             [1, 1, 1, 1, 1, 1, 1, 1],
    //         ],
    //     );
    // }
}
