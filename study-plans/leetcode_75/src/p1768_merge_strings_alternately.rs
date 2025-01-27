pub struct Solution;

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut merged_word = String::new();
        let len1 = word1.len();
        let len2 = word2.len();

        for (a, b) in word1.chars().zip(word2.chars()) {
            merged_word = format!("{}{}{}", merged_word, a, b);
        }

        if len1 > len2 {
            merged_word = merged_word + &word1[len2..]
        } else {
            merged_word = merged_word + &word2[len1..]
        }
        merged_word
    }
}

// impl Solution {
//     pub fn merge_alternately(word1: String, word2: String) -> String {
//         word1
//             .chars()
//             .zip(word2.chars())
//             .flat_map(|(a, b)| [a, b])
//             .chain(word1.chars().skip(word2.len()))
//             .chain(word2.chars().skip(word1.len()))
//             .collect()
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let word1 = "abc".to_string();
        let word2 = "pqr".to_string();
        let res = "apbqcr".to_string();
        assert_eq!(Solution::merge_alternately(word1, word2), res);
    }

    #[test]
    fn test2() {
        let word1 = "ab".to_string();
        let word2 = "pqrs".to_string();
        let res = "apbqrs".to_string();
        assert_eq!(Solution::merge_alternately(word1, word2), res);
    }

    #[test]
    fn test3() {
        let word1 = "abcd".to_string();
        let word2 = "pq".to_string();
        let res = "apbqcd".to_string();
        assert_eq!(Solution::merge_alternately(word1, word2), res);
    }
}
