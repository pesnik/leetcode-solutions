pub struct Solution;

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let vowels: Vec<char> = s.chars().filter_map(|char| match char.to_lowercase().next().unwrap() {
            'a' | 'e' | 'i' | 'o' | 'u' => Some(char),
            _ => None
        }).collect();

        let mut idx = vowels.len();
        s.chars()
            .map(|ch| match ch.to_lowercase().next().unwrap() {
                'a' | 'e' | 'i' | 'o' | 'u' => {
                    idx -= 1;
                    vowels[idx]
                },
                _ => ch
            })
        .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let ans = Solution::reverse_vowels("leetcode".to_string());
        assert_eq!(ans, "leotcede".to_string());
    }
}
