// 49.Group Anagrams -- medium

// Given an array of strings strs, group the anagrams together. You can return the answer in any order.

// An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.

// Example 1:

// Input: strs = ["eat","tea","tan","ate","nat","bat"]
// Output: [["bat"],["nat","tan"],["ate","eat","tea"]]
// Example 2:

// Input: strs = [""]
// Output: [[""]]
// Example 3:

// Input: strs = ["a"]
// Output: [["a"]]

// Constraints:

// 1 <= strs.length <= 104
// 0 <= strs[i].length <= 100
// strs[i] consists of lowercase English letters.

use std::collections::HashMap;
pub struct Solution {}

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        // Abnormal situations
        if strs.len() < 1 {
            panic!();
        }
        if strs.len() == 1 {
            return vec![vec![strs[0].clone()]];
        }

        let mut map: HashMap<String, Vec<String>> = HashMap::new();

        for word in strs {
            let mut char_arr = word.chars().collect::<Vec<_>>();
            char_arr.sort_by(|a, b| a.cmp(b));
            let sorted_word: String = char_arr.iter().collect();

            if let Some(arr) = map.get_mut(&sorted_word) {
                arr.push(word.clone());
            } else {
                map.insert(sorted_word, vec![word]);
            }
        }

        let mut res = vec![];
        map.iter().for_each(|(_k, v)| res.push(v.clone()));
        res
    }
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_anagrams() {
        let words = vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ];

        let expected = vec![
            vec!["ate".to_string(), "eat".to_string(), "tea".to_string()],
            vec!["nat".to_string(), "tan".to_string()], 
            vec!["bat".to_string()],
        ];

        let result =Solution:: group_anagrams(words);

        assert_eq!(result, expected);
    }
}