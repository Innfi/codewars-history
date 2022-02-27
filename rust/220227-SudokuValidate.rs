//https://www.codewars.com/kata/540afbe2dc9f615d5e000425

#[cfg(test)]
mod tests {
    struct Sudoku {
        data: Vec<Vec<u32>>,
    }
    
    
    // impl Sudoku{
    //     fn is_valid(&self) -> bool {
    //       // YOUR SOLUTION
    //       false
    //     }
    // }

    #[test]
    fn d2_indexing() {
        let input = Sudoku {
            data: vec![
                vec![7,8,4, 1,5,9, 3,2,6],
                vec![5,3,9, 6,7,2, 8,4,1],
                vec![6,1,2, 4,3,8, 7,5,9],

                vec![9,2,8, 7,1,5, 4,6,3],
                vec![3,5,7, 8,4,6, 1,9,2],
                vec![4,6,1, 9,2,3, 5,8,7],
                
                vec![8,7,6, 3,9,4, 2,1,5],
                vec![2,4,3, 5,6,1, 9,7,8],
                vec![1,9,5, 2,8,7, 6,3,4]
            ]
        };

        input.data.into_iter().for_each(|x| {
            let sum: u32 = x.into_iter().sum();

            assert_eq!(sum, 45);
        });
    }

    #[test]
    fn d2_vertical_indexing() {
        let input = Sudoku {
            data: vec![
                vec![7,8,4, 1,5,9, 3,2,6],
                vec![5,3,9, 6,7,2, 8,4,1],
                vec![6,1,2, 4,3,8, 7,5,9],

                vec![9,2,8, 7,1,5, 4,6,3],
                vec![3,5,7, 8,4,6, 1,9,2],
                vec![4,6,1, 9,2,3, 5,8,7],
                
                vec![8,7,6, 3,9,4, 2,1,5],
                vec![2,4,3, 5,6,1, 9,7,8],
                vec![1,9,5, 2,8,7, 6,3,4]
            ]
        };

        let vertical_sum_first: u32 = input.data.into_iter().map(|x| {
            *x.get(0).unwrap()
        }).sum();

        assert_eq!(vertical_sum_first, 45);
    }

    // #[test]
    // fn good_sudoku() {
    //     let good_sudoku_1 = Sudoku{
    //         data: vec![
    //                 vec![7,8,4, 1,5,9, 3,2,6],
    //                 vec![5,3,9, 6,7,2, 8,4,1],
    //                 vec![6,1,2, 4,3,8, 7,5,9],

    //                 vec![9,2,8, 7,1,5, 4,6,3],
    //                 vec![3,5,7, 8,4,6, 1,9,2],
    //                 vec![4,6,1, 9,2,3, 5,8,7],
                    
    //                 vec![8,7,6, 3,9,4, 2,1,5],
    //                 vec![2,4,3, 5,6,1, 9,7,8],
    //                 vec![1,9,5, 2,8,7, 6,3,4]
    //             ]
    //     };
        
    //     let good_sudoku_2 = Sudoku{
    //         data: vec![
    //                 vec![1, 4,  2, 3],
    //                 vec![3, 2,  4, 1],
            
    //                 vec![4, 1,  3, 2],
    //                 vec![2, 3,  1, 4],
    //             ]
    //     };
    //     assert!(good_sudoku_1.is_valid());
    //     assert!(good_sudoku_2.is_valid());
    // }

    // #[test]
    // fn bad_sudoku() {
    //     let bad_sudoku_1 = Sudoku{
    //         data: vec![
    //                 vec![1,2,3, 4,5,6, 7,8,9],
    //                 vec![1,2,3, 4,5,6, 7,8,9],
    //                 vec![1,2,3, 4,5,6, 7,8,9],

    //                 vec![1,2,3, 4,5,6, 7,8,9],
    //                 vec![1,2,3, 4,5,6, 7,8,9],
    //                 vec![1,2,3, 4,5,6, 7,8,9],
                    
    //                 vec![1,2,3, 4,5,6, 7,8,9],
    //                 vec![1,2,3, 4,5,6, 7,8,9],
    //                 vec![1,2,3, 4,5,6, 7,8,9],
    //             ]
    //     };
        
    //     let bad_sudoku_2 = Sudoku{
    //         data: vec![
    //                 vec![1,2,3,4,5],
    //                 vec![1,2,3,4],
    //                 vec![1,2,3,4],
    //                 vec![1],
    //             ]
    //     };
    //     assert!(!bad_sudoku_1.is_valid());
    //     assert!(!bad_sudoku_2.is_valid());
    // }
}
