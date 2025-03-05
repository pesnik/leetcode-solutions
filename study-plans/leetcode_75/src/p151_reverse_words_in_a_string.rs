pub struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_whitespace().rev().collect::<Vec<&str>>().join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample3() {
        let reversed_words = Solution::reverse_words("a good   example".to_string());
        assert_eq!(reversed_words, "example good a".to_string());
    }

    #[test]
    fn sample2() {
        let reversed_words = Solution::reverse_words("  hello world  ".to_string());
        assert_eq!(reversed_words, "world hello".to_string());
    }
}
