struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn largest_word_count(messages: Vec<String>, senders: Vec<String>) -> String {
        // let senders_init = senders.iter().map(|s| (s.clone(), 0))
        let counter_map: HashMap<&String, usize> =
            HashMap::with_capacity(senders.len().checked_div(2).unwrap_or_default());
        messages
            .iter()
            .zip(senders.iter())
            .map(|(m, s)| (s, m.chars().filter(|x| *x == ' ').count() + 1))
            .fold(counter_map, |mut acc, (s, word_cnt)| {
                *acc.entry(s).or_insert(0) += word_cnt;
                acc
            })
            .iter()
            .max_by(|x, y| x.1.cmp(y.1).then((*x.0).cmp(*y.0)))
            .map_or_else(|| String::new(), |v| (*v.0).clone())
    }
}

mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_example1() {
        let m = vec![
            "Hello userTwooo",
            "Hi userThree",
            "Wonderful day Alice",
            "Nice day userThree",
        ]
        .iter()
        .map(|v| String::from(*v))
        .collect();
        let s = vec!["Alice", "userTwo", "userThree", "Alice"]
            .iter()
            .map(|v| String::from(*v))
            .collect();
        assert_eq!(Solution::largest_word_count(m, s), String::from("Alice"))
    }

    #[test]
    fn test_example2() {
        let m = vec![
            "How is leetcode for everyone",
            "Leetcode is useful for practice",
        ]
        .iter()
        .map(|v| String::from(*v))
        .collect();
        let s = vec!["Bob", "Charlie"]
            .iter()
            .map(|v| String::from(*v))
            .collect();
        assert_eq!(Solution::largest_word_count(m, s), String::from("Charlie"))
    }

    // #[test]
    // fn test() {
    //     let y = &&String::from("Charlie");
    //     let x = &&String::from("Bob");

    //     println!("{:?}", (*x).cmp(*y))
    // }

    // #[test]
    // fn test2() {
    //     let x = &5_usize;
    //     let y = &234234_usize;
    //     println!("{:?}", x.cmp(y))
    // }
}
