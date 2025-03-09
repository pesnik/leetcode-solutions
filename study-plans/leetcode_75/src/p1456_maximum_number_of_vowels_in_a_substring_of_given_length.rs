pub struct Solution;

impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let k = k as usize;
        let vec = s.chars().collect::<Vec<_>>();
        let window = s
            .chars()
            .take(k)
            .filter(|ch| match ch {
                'a' | 'e' | 'i' | 'o' | 'u' => true,
                _ => false,
            })
            .collect::<Vec<_>>();

        let mut max_vowels = window.len() as i32;
        let mut prev_cnt = max_vowels;

        for i in k..s.len() {
            let mut vowels_in_curr_window =  match vec[i - k] {
                'a' | 'e' | 'i' | 'o' | 'u' => prev_cnt - 1,
                _ => prev_cnt,
            };

            vowels_in_curr_window +=  match vec[i] {
                'a' | 'e' | 'i' | 'o' | 'u' => 1,
                _ => 0,
            };

            max_vowels = max_vowels.max(vowels_in_curr_window);
            prev_cnt = vowels_in_curr_window;
        }

        max_vowels
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
