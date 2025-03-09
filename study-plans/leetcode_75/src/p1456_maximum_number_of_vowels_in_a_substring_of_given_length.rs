pub struct Solution;

impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let k = k as usize;
        let vec: Vec<char> = s.chars().collect();
        let is_vowel = |c: char| -> bool { matches!(c, 'a' | 'e' | 'i' | 'o' | 'u') };
        let mut current_vowels = vec.iter().take(k).filter(|&&ch| is_vowel(ch)).count();

        let mut max_vowels = current_vowels;

        for i in k..s.len() {
            if is_vowel(vec[i - k]) {
                current_vowels -= 1;
            }

            if is_vowel(vec[i]) {
                current_vowels += 1;
            }

            max_vowels = max_vowels.max(current_vowels);
        }

        max_vowels as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let result = Solution::max_vowels("abciiidef".to_string(), 3);
        assert_eq!(result, 3);
    }
}
