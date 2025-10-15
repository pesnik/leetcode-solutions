pub struct Solution;

impl Solution {
    pub fn longest_ones(_nums: Vec<i32>, _k: i32) -> i32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample2() {
        let max = Solution::longest_ones(vec![0,0,1,1,0,0,1,1,1,0,1,1,0,0,0,1,1,1,1], 3);
        assert_eq!(max, 10);
    }
}
