

struct Solution {}
// medium
use std::cmp::{max, min};
impl Solution {
    fn check(cnt: &[i32; 3], k: i32) -> bool {
        cnt[0] >= k && cnt[1] >= k && cnt[2] >= k
    }

    pub fn take_characters(s: String, k: i32) -> i32 {
        let mut cnt = [0; 3];

        if k == 0 {
            return 0;
        }

        let mut lend = 0;
        for (i, c) in s.bytes().enumerate() {
            if let b'a' | b'b' | b'c' = c {
                cnt[(c - b'a') as usize] += 1;
            }
            if cnt[0] >= k && cnt[1] >= k && cnt[2] >= k {
                lend = i + 1;
                break;
            }
        }
        if lend <= 0 { return -1; }

        let bytes: Vec<u8>  = s.bytes().collect();
        let blen = bytes.len();


        let mut l = lend;
        let mut r = blen;
        let mut res = lend;
        loop {
            if Self::check(&cnt, k) {
                res = min(l + blen - r, res);
                if l == 0 { break; }
                l -= 1;
                cnt[(bytes[l as usize] - b'a') as usize] -= 1;
            } else {
                r -= 1;
                cnt[(bytes[r as usize] - b'a') as usize] += 1;
            }
        }
        return res as i32;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(Solution::take_characters(String::from("aabaaaacaabc"), 2), 8);
    }

    #[test]
    fn test_example2() {
        assert_eq!(Solution::take_characters(String::from("a"), 1), -1);
    }

    #[test]
    fn test_example3() {
        assert_eq!(Solution::take_characters(String::from("cbbac"), 1), 3);
    }

}



