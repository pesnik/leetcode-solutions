pub struct Solution;

impl Solution {
    pub fn gcd(a: usize, b: usize) -> usize {
        if b == 0 {
            return a;
        }

        Self::gcd(b, a % b)
    }

    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        if format!("{}{}", str1, str2) != format!("{}{}", str2, str1) {
            return String::from("");
        }

        let len1 = str1.len();
        let len2 = str2.len();
        let gcd_len = Self::gcd(len1, len2);

        str1[..gcd_len].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let str1 = "ABCABC".to_string();
        let str2 = "ABC".to_string();

        assert_eq!(Solution::gcd_of_strings(str1, str2), "ABC");

        let str1 = String::from("ABABAB");
        let str2 = String::from("ABAB");
        assert_eq!(Solution::gcd_of_strings(str1, str2), "AB");

        let str1 = String::from("LEET");
        let str2 = String::from("CODE");
        assert_eq!(Solution::gcd_of_strings(str1, str2), "");
    }
}
