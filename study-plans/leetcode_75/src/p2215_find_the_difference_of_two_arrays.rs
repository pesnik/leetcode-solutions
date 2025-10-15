use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let set1: HashSet<i32> = nums1.into_iter().collect();
        let set2: HashSet<i32> = nums2.into_iter().collect();
        
        vec![
            set1.difference(&set2).copied().collect(),
            set2.difference(&set1).copied().collect(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let distinct_integers = Solution::find_difference(vec![1,2,3], vec![2,4,6]);
        assert_eq!(distinct_integers, vec![[1,3],[4,6]]);
    }
}
