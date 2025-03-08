pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let len = height.len();
        let mut left = 0;
        let mut right = len - 1;
        let mut max_capacity = height[left].min(height[right]) * right as i32;
        while left < right {
            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
            let cap = height[left].min(height[right]) * ((right - left) as i32);
            max_capacity = max_capacity.max(cap);
        }

        max_capacity
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let max_area = Solution::max_area([1,8,6,2,5,4,8,3,7].to_vec());
        assert_eq!(max_area, 49);
    }

    #[test]
    fn custom1() {
        let max_area = Solution::max_area([1,3,2,5,25,24,5].to_vec());
        assert_eq!(max_area, 24);
    }
}
