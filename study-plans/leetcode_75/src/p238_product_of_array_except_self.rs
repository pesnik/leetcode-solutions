pub struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut forward_prod: Vec<i32> = Vec::from([]);
        let mut backward_prod: Vec<i32> = nums.iter().rev().map(|&i| i).collect();

        forward_prod.push(nums[0]);
        for i in 1..nums.len() {
            forward_prod.push(forward_prod[i - 1] * nums[i]);
        }

        for i in 1..nums.len() {
            backward_prod[i] = backward_prod[i - 1] * backward_prod[i];
        }

        let mut selfless_prod: Vec<i32> = Vec::from([]);
        backward_prod.reverse();
        selfless_prod.push(backward_prod[1]);
        for i in 1..=nums.len() - 2 {
            selfless_prod.push(forward_prod[i - 1] * backward_prod[i + 1]);
        }
        selfless_prod.push(forward_prod[nums.len() - 2]);
        selfless_prod
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let product = Solution::product_except_self([1, 2, 3, 4].to_vec());
        assert_eq!(product, [24, 12, 8, 6].to_vec());
    }

    #[test]
    fn sample2() {
        let product = Solution::product_except_self([-1, 1, 0, -3, 3].to_vec());
        assert_eq!(product, [0, 0, 9, 0, 0].to_vec());
    }
}
