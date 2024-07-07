struct Solution {}
// easy
use std::cmp::min;
impl Solution {
    pub fn k_items_with_maximum_sum(
        num_ones: i32,
        num_zeros: i32,
        num_neg_ones: i32,
        k: i32,
    ) -> i32 {
        [(num_ones, 1), (num_zeros, 0), (num_neg_ones, -1)]
            .iter()
            .fold((0, k), |(res, resk), (n, v)| match resk {
                0 => (res, resk),
                _ => {
                    let n_avai = min(resk, *n);
                    (res + n_avai * v, resk - n_avai)
                }
            })
            .0
    }
}
