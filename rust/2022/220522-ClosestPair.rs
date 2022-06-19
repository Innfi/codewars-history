//https://www.codewars.com/kata/5376b901424ed4f8c20002b7

/*
use itertools::Itertools;

fn closest_pair(points: &[(f64, f64)]) -> ((f64, f64), (f64, f64)) {
    let n = points.len();
    let mut l = f64::MAX;
    let mut a = 0;
    let mut b = 0;
    let mut tolerance = l.sqrt();
    let arr: Vec<(f64, f64)> = points.iter()
                                     .cloned()
                                     .sorted_by(|&a,&b| (a.0)
                                     .partial_cmp(&b.0).unwrap())
                                     .collect();
    for i in 0..(n-1) {
        for j in (i+1)..n {
            if arr[j].0 >= arr[i].0 + tolerance {
                break
            } else {
                let ls = (arr[i].0 - arr[j].0).powf(2.0) + (arr[i].1 - arr[j].1).powf(2.0);
                if ls < l {
                    l = ls;
                    tolerance = l.sqrt();
                    a = i;
                    b = j;
                }
            }
        } 
    }
    
    (arr[a], arr[b])
}
*/

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn test_powi() {
        let input: f64 = -2.0;

        assert_eq!(input.powi(2), 4.0);
    }

    fn to_distance(lhs: (f64, f64), rhs: (f64, f64)) -> f64 {
        ( (rhs.0 - lhs.0).powi(2) + (rhs.1 - lhs.1).powi(2) ).sqrt()
    }

    #[test]
    fn test_to_distance() {
        let point_a: (f64, f64) = (6.0, 6.0);
        let point_b: (f64, f64) = (2.0, 3.0);

        assert_eq!(to_distance(point_a, point_b), 5.0);
    }

    fn closest_pair(points: &[(f64, f64)]) -> ((f64, f64), (f64, f64)) {
        let mut distance_map: HashMap<(usize, usize), f64> = HashMap::new();

        let mut min_distance: f64 = std::f64::MAX;
        let mut output: ((f64, f64), (f64, f64)) = ((0.0, 0.0), (0.0, 0.0));

        for x in 0..points.len() {
            for y in 0..points.len() {
                if x == y { continue; }
                
                let indexes: (usize, usize) = if x < y { (x,y) } else { (y,x) };

                if distance_map.contains_key(&(indexes.0, indexes.1)) { continue; }

                let distance = to_distance(points[indexes.0], points[indexes.1]);

                distance_map.insert((indexes.0, indexes.1), distance);

                if min_distance > distance {
                    min_distance = distance;
                    output = (points[indexes.0], points[indexes.1]);
                }
            }
        }

        output
    }

    type Points = ((f64, f64), (f64, f64));
    
    fn verify(actual: Points, expected: Points) {
        if actual == expected || (actual.0 == expected.1 && actual.1 == expected.0) {
            assert!(true)
        } else {
            assert_eq!(actual, expected)
        }
    }
    
    #[test]
    fn sample_tests() {
        verify(
            closest_pair(&[(2.0, 2.0), (6.0, 3.0)]),
            ((2.0, 2.0), (6.0, 3.0))
        );
        verify(
            closest_pair(&[(2.0, 2.0), (2.0, 8.0), (5.0, 5.0), (6.0, 3.0),
                           (6.0, 7.0), (7.0, 4.0), (7.0, 9.0)]),
            ((6.0, 3.0), (7.0, 4.0))
        );
        verify(
            closest_pair(&[(2.0, 2.0), (2.0, 8.0), (5.0, 5.0), (5.0, 5.0), 
                           (6.0, 3.0), (6.0, 7.0), (7.0, 4.0), (7.0, 9.0)]),
            ((5.0, 5.0), (5.0, 5.0)));
    }
}
