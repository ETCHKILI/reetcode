use std::collections::HashMap;




struct Solution {}
// base on Solution 1, dont use vectors of height, but hashmap of height
// medium
impl Solution {
    fn build_xsh(recs: Vec<Vec<i32>>) -> HashMap<i32, Vec<i32>> {
        let mut res = HashMap::new();
        for rec in recs.iter() {
            res.entry(rec[1]).or_insert(vec![]).push(rec[0])
        }
        for entry in res.values_mut() {
            entry.sort_unstable();
        }
        res
    }

    pub fn count_rectangles(rectangles: Vec<Vec<i32>>, points: Vec<Vec<i32>>) -> Vec<i32> {
        let xsh = Self::build_xsh(rectangles);

        points
            .iter()
            .map(|p| {
                let (px, py) = (p[0], p[1]);
                let mut cnt = 0;
                for h in py..=100 {
                    if let Some(xs) = xsh.get(&h) {
                        cnt += xs.len() - xs.binary_search(&px).map_or_else(|e| e, |v| v)
                    }
                }
                cnt as i32
            })
            .collect()
    }
}

struct Solution2 {}
// TLE
impl Solution2 {
    pub fn count_rectangles(rectangles: Vec<Vec<i32>>, points: Vec<Vec<i32>>) -> Vec<i32> {
        let mut recs_sort_by_x: Vec<&Vec<i32>> = rectangles.iter().collect();
        // let mut xi_by_ref = HashMap::with_capacity(recs_sort_by_x.len()); 
        recs_sort_by_x.sort_unstable_by(|a, b| a[0].cmp(&b[0]).then(a[1].cmp(&b[1])));
        // for (i, rec) in recs_sort_by_x.iter().enumerate() {
        //     xi_by_ref.insert(*rec, i);
        // }

        let mut recs_sort_by_y: Vec<&Vec<i32>> = rectangles.iter().collect();
        let mut yi_by_ref = std::collections::HashMap::with_capacity(recs_sort_by_y.len()); 
        recs_sort_by_y.sort_unstable_by(|a, b| a[1].cmp(&b[1]).then(a[0].cmp(&b[0])));
        for (i, rec) in recs_sort_by_y.iter().enumerate() {
            yi_by_ref.insert(*rec, i);
        }



        points
            .iter()
            .map(
                |p| {
                    let xi = recs_sort_by_x
                        .binary_search_by(|rec| rec[0].cmp(&p[0]).then(rec[1].cmp(&p[1])))
                        .map_or_else(|e| e, |v| v);
                    let yi = recs_sort_by_y
                        .binary_search_by(|rec| rec[1].cmp(&p[1]).then(rec[0].cmp(&p[0])))
                        .map_or_else(|e| e, |v| v);
                        // .unwrap();
                    let mut cnt = 0;
                    for rec in recs_sort_by_x[xi..].iter() {
                        if yi_by_ref[rec] >= yi {
                            cnt += 1;
                        }
                    }
                    cnt 
                }
            )
            .collect()
    }
}

struct Solution1 {}
// TLE
impl Solution1 {
    fn build_dp_matrix(recs: &mut Vec<(i32, i32)>) -> Vec<Vec<(i32, usize)>> {
        let mut res: Vec<Vec<(i32, usize)>> = vec![vec![]; 101];
        for rec in recs {
            for i in (0..(rec.1 + 1).try_into().unwrap_or_default()).rev() {
                if let Some(xbs_of_y) = res.get_mut(i) {
                    match xbs_of_y.binary_search_by(|e| e.0.cmp(&rec.0)) {
                        Ok(xi) => {
                            for j in 0..=xi {
                                let (xj, cnt) = xbs_of_y[j];
                                xbs_of_y[j] = (xj, cnt + 1); 
                            }
                        },
                        Err(xi) => {
                            let next_cnt = xbs_of_y.get(xi).map(|x| x.1).unwrap_or_default();
                            xbs_of_y.insert(xi, (rec.0, next_cnt));
                            for j in 0..=xi {
                                let (xj, cnt) = xbs_of_y[j];
                                xbs_of_y[j] = (xj, cnt + 1); 
                            }
                        },
                    };
                }
            }
        }
        res
    }

    fn search_dp_matrix(dp_matrix: &Vec<Vec<(i32, usize)>>, point: (i32, i32)) -> usize {
        dp_matrix
            .get(point.1 as usize)
            .map(|x_b_cnt| {
                let xi = x_b_cnt.binary_search_by(|e| e.0.cmp(&point.0) ).map_or_else(|e| e, |v| v);
                x_b_cnt.get(xi).map(|x| x.1).unwrap_or_default()
            })
            .unwrap_or_default()
        
    }

    pub fn count_rectangles(rectangles: Vec<Vec<i32>>, points: Vec<Vec<i32>>) -> Vec<i32> {
        let mut recs: Vec<_> = rectangles.iter().map(|rec| (rec[0], rec[1])).collect();
        recs.sort_unstable_by(|x, y| x.1.cmp(&y.1).reverse().then(x.0.cmp(&y.0).reverse()));
        let dp_mat = Self::build_dp_matrix(&mut recs);

        points
            .iter()
            .map(|p| {
                Self::search_dp_matrix(&dp_mat, (p[0], p[1])) as i32
            })
            .collect()
    }

    
}

#[cfg(test)]
mod tests {
    use std::hash::{DefaultHasher, Hash};

    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(Solution::count_rectangles(vec![vec![1,2],vec![2,3],vec![2,5]], vec![vec![2,1],vec![1,4]]), [2,1]);
    }

    #[test]
    fn test_example2() {
        assert_eq!(Solution::count_rectangles(vec![vec![1,1],vec![2,2],vec![3,3]], vec![vec![1,3],vec![1,1]]), [1,3]);
    }

    #[test]
    fn test_example3() {
        assert_eq!(
            Solution::count_rectangles(
                vec![vec![7,1],vec![2,6],vec![1,4],vec![5,2],vec![10,3],vec![2,4],vec![5,9]], 
                vec![vec![10,3],vec![8,10],vec![2,3],vec![5,4],vec![8,5],vec![7,10],vec![6,6],vec![3,6]] 
            ), 
            [1,0,4,1,0,0,0,1]
        );
    }

    #[test]
    fn test_example4() {
        let mut hasher = DefaultHasher::new();
        assert_eq!((&vec![3; 3]).hash(&mut hasher), (&vec![3, 3, 3]).hash(&mut hasher));
    }

}
