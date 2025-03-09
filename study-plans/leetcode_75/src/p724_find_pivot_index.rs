pub struct Solution;

impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let len = nums.len() - 1;
        let prefix_sum: Vec<i32> = nums
            .iter()
            .scan(0, |sum, &n| {
                *sum += n;
                Some(*sum)
            })
            .collect();

        // println!("{:?}", prefix_sum);
        let total_sum = prefix_sum.last().unwrap();
        if total_sum - nums[0] == 0 {
            return 0;
        }

        for i in 1..=len {
            let lsum = prefix_sum[i - 1];
            let rsum = total_sum - prefix_sum[i];

            if lsum == rsum {
                return i as i32;
            }
        }

        return -1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let index = Solution::pivot_index([1, 7, 3, 6, 5, 6].to_vec());
        assert_eq!(index, 3);
    }

    #[test]
    fn sample2() {
        let index = Solution::pivot_index([1, 2, 3].to_vec());
        assert_eq!(index, -1);
    }

    #[test]
    fn sample3() {
        let index = Solution::pivot_index([2, 1, -1].to_vec());
        assert_eq!(index, 0);
    }

    #[test]
    fn sample4() {
        let index = Solution::pivot_index([-1, -1, 0, 1, 1, 0].to_vec());
        assert_eq!(index, 5);
    }
}
