//https://www.codewars.com/kata/534e01fbbb17187c7e0000c6

/*
fn spiralize(size: usize) -> Vec<Vec<i8>> {
    let mut spiral = vec![vec![0; size]; size];
    let mut value = 1;
    
    for j in 0..(size + 1) / 2 {
        for i in j..(size - j) {
            spiral[i][j] = value;
            spiral[j][i] = value;

            spiral[i][size - 1 - j] = value;
            spiral[size - 1 - j][i] = value;
        }

        value = (value + 1) % 2;
        
        if j < (size - 1) / 2 || spiral[j][j - 1] == 1 {
            spiral[j + 1][j] = value;
        }
    }

    spiral
}
*/

#[cfg(test)]
mod tests {
    #[test]
    fn create_array2d() {
        let array_2d: Vec<Vec<i8>> = vec![ vec![0; 10]; 10 ];

        array_2d.into_iter().for_each(|x| {
            assert_eq!(x.into_iter().any(|element| element == 0), true);
        });
    }

    fn mark_right(
        array_2d: &mut Vec<Vec<i8>>,
        x: &mut usize,
        y: &mut usize,
        len: usize) {

        let mut temp = 0;
        for i in *x..len {
            if i+1 <= len-1 && array_2d[*y][i+1] == 1 { break; }

            array_2d[*y][i] = 1;
            temp = i;
        }

        *x = temp;
    }

    fn mark_down(
        array_2d: &mut Vec<Vec<i8>>, 
        x: &mut usize,
        y: &mut usize,
        len: usize) {

        let mut temp = 0;
        for i in *y..len {
            if i+1 <= len-1 && array_2d[i+1][*x] == 1 { break; }
            
            array_2d[i][*x] = 1;
            temp = i;
        }

        *y = temp;
    }

    fn mark_left(
        array_2d: &mut Vec<Vec<i8>>, 
        x: &mut usize,
        y: &mut usize,
        len: usize) {

        let mut temp = 0;
        for i in 0..len {
            if *x-i >= 1 && array_2d[*y][*x-i-1] == 1 { break; }

            array_2d[*y][*x-i] = 1;

            temp = *x-i;
        }

        *x = temp;
    }

    fn mark_up(
        array_2d: &mut Vec<Vec<i8>>, 
        x: &mut usize,
        y: &mut usize,
        len: usize) {

        let mut temp = 0;
        for i in 0..len {
            if *y-i >= 1 && array_2d[*y-i-1][*x] == 1 { break; }

            array_2d[*y-i][*x] = 1;
            temp = *y-i;
        }

        *y = temp;
    }

    #[test]
    fn traverse_array2d() {
        let len = 10;
        let mut array_2d: Vec<Vec<i8>> = vec![ vec![0; len]; len ];
        let mut x = 0;
        let mut y = 0;

        mark_right(&mut array_2d, &mut x, &mut y, len);
        mark_down(&mut array_2d, &mut x, &mut y, len);
        mark_left(&mut array_2d, &mut x, &mut y, len);
        mark_up(&mut array_2d, &mut x, &mut y, len);

        mark_right(&mut array_2d, &mut x, &mut y, len);
        mark_down(&mut array_2d, &mut x, &mut y, len);
        mark_left(&mut array_2d, &mut x, &mut y, len);
        mark_up(&mut array_2d, &mut x, &mut y, len);
        
        
        assert_eq!(array_2d.len(), 10);

        // array_2d.into_iter().for_each(|x: Vec<i8>| {
        //     x.into_iter().for_each(|e| {
        //         print!("{} ", e);
        //     });
        //     println!("");
        // });
    }

    // fn spiralize(size: usize) -> Vec<Vec<i8>> {

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
