struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
        let mh = mat.len();
        let mw = mat.get(0).map(|row| row.len()).unwrap_or_default();

        let mut rcnt = vec![mw; mh];
        let mut ccnt = vec![mh; mw];

        let yx_by_v: HashMap<&i32, (usize, usize)> = mat.iter().flatten().enumerate().map( |(i, v)| (v, (i / mw, i % mw))).collect();

        // let mut yx_by_v = HashMap::new();
        // for (y, row) in mat.iter().enumerate() {
        //     for (x, v) in row.iter().enumerate() {
        //         yx_by_v.insert(v, (y, x));
        //     }
        // }

        for (i, a) in arr.iter().enumerate() {
            let (ay, ax) = yx_by_v[&a];
            
            rcnt[ay] -= 1;
            ccnt[ax] -= 1;
            
            if rcnt[ay] <= 0 || ccnt[ax] <= 0 {
                return i as i32;
            }
        }
        42
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let arr = vec![1,3,4,2];
        let mat = vec![vec![1,4],vec![2,3]];
        assert_eq!(Solution::first_complete_index(arr, mat), 2)
    }

    #[test]
    fn test_example2() {
        let arr = vec![2,8,7,4,1,3,5,6,9];
        let mat = vec![vec![3,2,5],vec![1,4,6],vec![8,7,9]];
        assert_eq!(Solution::first_complete_index(arr, mat), 3)
    }

    #[test]
    fn test_example3() {
        let arr = vec![8,2,4,9,3,5,7,10,1,6];
        let mat = vec![vec![8,2,9,10,4],vec![1,7,6,3,5]];
        assert_eq!(Solution::first_complete_index(arr, mat), 5)
    }
}