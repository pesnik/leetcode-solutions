pub struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut nonzeroes = nums
            .iter()
            .filter_map(|&num| match num {
                0 => None,
                _ => Some(num),
            })
            .collect::<Vec<i32>>();
        let zeroes: Vec<i32> = vec![0; nums.len() - nonzeroes.len()];
        nonzeroes.extend(zeroes);
        *nums = nonzeroes;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let mut nums = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 3, 12, 0, 0]);
    }
}
