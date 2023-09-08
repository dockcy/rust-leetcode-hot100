// 3. Longest Substring Without Repeating Characters

// Given a string s, find the length of the longest
// substring
//  without repeating characters.

// Example 1:

// Input: s = "abcabcbb"
// Output: 3
// Explanation: The answer is "abc", with the length of 3.
// Example 2:

// Input: s = "bbbbb"
// Output: 1
// Explanation: The answer is "b", with the length of 1.
// Example 3:

// Input: s = "pwwkew"
// Output: 3
// Explanation: The answer is "wke", with the length of 3.
// Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.

// Constraints:

// 0 <= s.length <= 5 * 10^4
// s consists of English letters, digits, symbols and spaces.

pub struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.len() <= 1 {
            return s.len() as i32;
        }
        let chars = s.chars().collect::<Vec<_>>();
        let mut map = HashMap::new();
        let mut i = 0;
        let mut max_len = 0;
        while i < chars.len() {
            if let Some(&index) = map.get(&chars[i]) {
                max_len = max_len.max(map.len());
                i = index + 1;
                map.clear();
            } else {
                map.insert(&chars[i], i);
                i += 1;
            }
        }
        max_len.max(map.len()) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let s = "asdwdsawlkwa";
        let s1 = "au";
        let s3 = "";
        let s4 = "abcabcbb";
        assert_eq!(6,Solution::length_of_longest_substring(s.to_string()));
        assert_eq!(2,Solution::length_of_longest_substring(s1.to_string()));
        assert_eq!(0,Solution::length_of_longest_substring(s3.to_string()));
        assert_eq!(3,Solution::length_of_longest_substring(s4.to_string()));
        
    }
}
