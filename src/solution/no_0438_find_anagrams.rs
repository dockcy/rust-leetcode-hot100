// 438. Find All Anagrams in a String

// Given two strings s and p, return an array of all the start indices of p's anagrams in s. You may return the answer in any order.

// An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.

// Example 1:

// Input: s = "cbaebabacd", p = "abc"
// Output: [0,6]
// Explanation:
// The substring with start index = 0 is "cba", which is an anagram of "abc".
// The substring with start index = 6 is "bac", which is an anagram of "abc".
// Example 2:

// Input: s = "abab", p = "ab"
// Output: [0,1,2]
// Explanation:
// The substring with start index = 0 is "ab", which is an anagram of "ab".
// The substring with start index = 1 is "ba", which is an anagram of "ab".
// The substring with start index = 2 is "ab", which is an anagram of "ab".

// Constraints:

// 1 <= s.length, p.length <= 3 * 104
// s and p consist of lowercase English letters.

pub struct Solution {}

use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

impl Solution {

    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let mut cnt = vec![0; 128];
        p.chars().for_each(|c| cnt[c as usize] += 1);
        let mut ans = vec![];

        let s = s.chars().collect::<Vec<char>>();
        let (mut low, mut high) = (0, 0);
        while high < s.len() {
            let index = s[high] as usize;
            if cnt[index] > 0 {
                cnt[index] -= 1;
                high += 1;
                if high - low == p.len() {
                    ans.push(low as i32);
                }
            } else {
                cnt[s[low] as usize] += 1;
                low += 1;
            }
        }
        ans
    }

   
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let s = "cbaebabacd";
        let p = "abc";
        assert_eq!(vec![0,6],Solution::find_anagrams(s.to_string(), p.to_string()));
        
    }
}
