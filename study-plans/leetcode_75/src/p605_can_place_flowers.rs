pub struct Solution;

impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let l = flowerbed.len();
        let f: i32 = flowerbed.iter().sum();
        if f + n > l.div_ceil(2) as i32 {
            return false;
        }

        if (l == 1 && f + n == 1 && flowerbed[0] == 0) || n == 0 {
            return true;
        }

        let mut placed = n;
        let bed_size = flowerbed.len();
        let mut reference = flowerbed.clone();
        for i in 0..bed_size {
            if flowerbed[i] == 1 {
                continue;
            }

            if i == 0 && i + 1 != bed_size && reference[i + 1] == 0 {
                placed -= 1;
                reference[i] = 1;
            } else if i != 0 && i + 1 != bed_size && reference[i - 1] == 0 && reference[i + 1] == 0 {
                placed -= 1;
                reference[i] = 1;
            } else if i + 1 == bed_size && reference[i] == 0 && reference[i - 1] == 0 {
                placed -= 1;
            }

            if placed == 0 {
                return true;
            }
        }
        return placed == 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let answer = Solution::can_place_flowers([1,0,0,0,1].to_vec(), 1);
        assert_eq!(answer, true);
    }

    #[test]
    fn sample2() {
        let answer = Solution::can_place_flowers([1,0,0,0,1].to_vec(), 2);
        assert_eq!(answer, false);
    }

    #[test]
    fn case1() {
        let answer = Solution::can_place_flowers([0,1,0,0,1].to_vec(), 1);
        assert_eq!(answer, false);
    }

    #[test]
    fn case2() {
        let answer = Solution::can_place_flowers([0,0,0,0,0].to_vec(), 3);
        assert_eq!(answer, true);
    }

    #[test]
    fn case3() {
        let answer = Solution::can_place_flowers([0,0,0,0,0].to_vec(), 4);
        assert_eq!(answer, false);
    }

    #[test]
    fn case4() {
        let answer = Solution::can_place_flowers([1, 0, 0].to_vec(), 1);
        assert_eq!(answer, true);

        let answer = Solution::can_place_flowers([1, 0, 0].to_vec(), 2);
        assert_eq!(answer, false);
    }

    #[test]
    fn case5() {
        let answer = Solution::can_place_flowers([1].to_vec(), 0);
        assert_eq!(answer, true);
    }
}
