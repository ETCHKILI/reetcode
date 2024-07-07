struct Solution {}

impl Solution {
    pub fn fib(n: i32) -> i32 {
        let un: usize = n as usize;
        let mut cache = vec![-1; 10000_usize];
        cache[0] = 0;
        cache[1] = 1;
        Self::fib_with_cache(un, &mut cache)
    }

    pub fn fib_with_cache(n: usize, cache: &mut Vec<i32>) -> i32 {
        match cache.get(n) {
            Some(-1) => {
                let res = Self::fib_with_cache(n - 1, cache) + Self::fib_with_cache(n - 2, cache);
                if let Some(elem) = cache.get_mut(n) {
                    *elem = res;
                }
                res
            }
            Some(x) => *x,
            None => {
                cache.extend(vec![-1_i32; n].iter());
                let res = Self::fib_with_cache(n - 1, cache) + Self::fib_with_cache(n - 2, cache);
                if let Some(elem) = cache.get_mut(n) {
                    *elem = res;
                }
                res
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(Solution::fib(2), 1);
    }

    #[test]
    fn test_example2() {
        assert_eq!(Solution::fib(3), 2);
    }

    #[test]
    fn test_example3() {
        assert_eq!(Solution::fib(4), 3);
    }

    #[test]
    fn test_example4() {
        assert_eq!(Solution::fib(39), 63245986);
    }
}
