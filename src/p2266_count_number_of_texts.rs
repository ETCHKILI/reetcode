use std::cmp::max;

struct Solution {}
// medium
impl Solution {
    pub fn count_texts(pressed_keys: String) -> i32 {
        let l = pressed_keys.len();
        let mut fx: Vec<i32> = Vec::with_capacity(l + 1);
        fx.push(1);

        let mut start = 0;
        let mut pre = 'x';
        for (i, c) in pressed_keys.char_indices() {
            if pre != c {
                start = i;
            };
            let check_step = match c {
                _ if c == '7' || c == '9' => 3,
                _ => 2,
            };
            let local_start = max(i.checked_sub(check_step).unwrap_or_default(), start);
            pre = c;
            let res = (fx[local_start..=i])
                .iter()
                .fold(0, |acc, e| (acc + *e) % Self::MOD);
            fx.push(res)
        }

        *fx.get(l).unwrap()
    }

    const MOD: i32 = 1_000_000_000 + 7;
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(Solution::count_texts(String::from_str("22233").unwrap()), 8);
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            Solution::count_texts(
                String::from_str("222222222222222222222222222222222222").unwrap()
            ),
            82876089
        );
    }
}
