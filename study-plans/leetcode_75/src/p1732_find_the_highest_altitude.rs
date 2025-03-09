pub struct Solution;

impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut largest = 0;
        let mut carry = 0;
        for i in 0..gain.len() {
            carry += gain[i];
            largest = largest.max(carry);
        }

        largest
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let largest = Solution::largest_altitude([-5, 1, 5, 0, -7].to_vec());
        assert_eq!(largest, 1);
    }

    #[test]
    fn sample2() {
        let largest = Solution::largest_altitude([-4, -3, -2, -1, 4, 3, 2].to_vec());
        assert_eq!(largest, 0);
    }
}
