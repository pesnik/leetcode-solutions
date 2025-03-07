pub struct Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut tortoise = 0;
        let checker: Vec<_> = s.chars().collect();
        for ch in t.chars() {
            if tortoise == s.len() {
                return true;
            }

            if ch == checker[tortoise] {
                tortoise += 1;
            }
        }

        tortoise == s.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let subsequence = Solution::is_subsequence("abc".to_string(), "ahbgdc".to_string());
        assert_eq!(subsequence, true);
    }
}
