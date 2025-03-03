pub struct Solution;

impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let max: i32 = candies.iter().copied().fold(0i32, i32::max);

        candies
            .iter()
            .map(|candy| {
                if candy + extra_candies >= max {
                    true
                } else {
                    false
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let response: Vec<bool> = Solution::kids_with_candies([2, 3, 5, 1, 3].to_vec(), 3);
        assert_eq!(response, [true, true, true, false, true].to_vec());
    }
}
