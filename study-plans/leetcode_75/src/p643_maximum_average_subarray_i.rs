pub struct Solution;

impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mut window: i32 = nums.iter().take(k as usize).sum();
        let mut max_avg = window as f64 / k as f64;
        for i in k as usize..nums.len() {
            window = window - nums[i - k as usize] + nums[i];
            max_avg = max_avg.max(window as f64 / k as f64);
        }

        max_avg
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let mx_average = Solution::find_max_average(vec![1, 12, -5, -6, 50, 3], 4);
        assert_eq!(mx_average, 12.75000);
    }
}
