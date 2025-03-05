pub struct Solution;

impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let mut store_keeper = vec![];
        let mut compressed_str = String::from("");
        for i in 0..chars.len() {
            if store_keeper.len() == 0 {
                store_keeper.push(chars[i]);
            } else if store_keeper[0] == chars[i] {
                store_keeper.push(chars[i]);
            } else {
                compressed_str = match store_keeper.len() {
                    1 => format!(
                        "{}{}",
                        compressed_str,
                        store_keeper[0].to_string()
                    ),
                    _ => format!(
                        "{}{}{}",
                        compressed_str,
                        store_keeper[0].to_string(),
                        store_keeper.len().to_string()
                    )
                };
                store_keeper.clear();
                store_keeper.push(chars[i]);
            }
        }

        if !store_keeper.is_empty() {
            compressed_str = match store_keeper.len() {
                1 => format!(
                    "{}{}",
                    compressed_str,
                    store_keeper[0].to_string()
                ),
                _ => format!(
                    "{}{}{}",
                    compressed_str,
                    store_keeper[0].to_string(),
                    store_keeper.len().to_string()
                )
            };
        }
        *chars = compressed_str.chars().collect::<Vec<char>>();
        compressed_str.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let mut input: Vec<char> = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];
        let compressed = Solution::compress(&mut input);
        assert_eq!(compressed, 6);
    }

    #[test]
    fn sample3() {
        let mut input: Vec<char> = vec!['a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b'];
        let compressed = Solution::compress(&mut input);
        assert_eq!(compressed, 4);
    }
}
