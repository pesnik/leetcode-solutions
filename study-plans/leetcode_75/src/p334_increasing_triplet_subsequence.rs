pub struct Solution;

impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        if nums.len() < 3 {
            return false;
        }

        let mut smallest = i32::MAX;
        let mut second_smallest = i32::MAX;

        for curr in nums {
            if curr <= smallest {
                smallest = curr;
            } else if curr <= second_smallest {
                second_smallest = curr;
            } else {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let triplet_existence = Solution::increasing_triplet(vec![2, 1, 5, 0, 4, 6]);
        assert_eq!(triplet_existence, true);
    }

    #[test]
    fn sample2() {
        let triplet_existence = Solution::increasing_triplet(vec![1, 2, 3, 4, 5]);
        assert_eq!(triplet_existence, true);
    }

    #[test]
    fn sample3() {
        let triplet_existence = Solution::increasing_triplet(vec![5, 4, 3, 2, 1]);
        assert_eq!(triplet_existence, false);
    }

    #[test]
    fn sample4() {
        let triplet_existence = Solution::increasing_triplet(vec![0, 4, 2, 1, 0, -1, -3]);
        assert_eq!(triplet_existence, false);
    }
}
