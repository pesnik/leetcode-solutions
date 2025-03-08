pub struct Solution;

use std::cmp::Ordering;

impl Solution {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut numbers = nums.clone();
        numbers.sort();
        let mut left = 0;
        let mut right = nums.len() - 1;
        let mut ops = 0;

        while left < right {
            (left, right, ops) = match (numbers[left] + numbers[right]).cmp(&k) {
                Ordering::Greater => (left, right - 1, ops),
                Ordering::Less => (left + 1, right, ops),
                _ => (left + 1, right - 1, ops + 1),
            };
        }

        ops
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let op = Solution::max_operations(vec![1,2,3,4], 5);
        assert_eq!(op, 2);
    }

    #[test]
    fn sample2() {
        let op = Solution::max_operations(vec![3, 1, 3, 4, 3], 6);
        assert_eq!(op, 1);
    }
}
