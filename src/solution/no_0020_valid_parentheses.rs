// 20. Valid Parentheses
// 简单
// 相关标签
// 相关企业
// 提示
// Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.
//
// An input string is valid if:
//
// Open brackets must be closed by the same type of brackets.
// Open brackets must be closed in the correct order.
// Every close bracket has a corresponding open bracket of the same type.
//
//
// Example 1:
//
// Input: s = "()"
// Output: true
// Example 2:
//
// Input: s = "()[]{}"
// Output: true
// Example 3:
//
// Input: s = "(]"
// Output: false
//
//
// Constraints:
//
// 1 <= s.length <= 10^4
// s consists of parentheses only '()[]{}'.

pub struct Solution {}


use std::collections::HashMap;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];
        let map = HashMap::from([
            (')', '('),
            (']', '['),
            ('}', '{')]);
        for c in s.chars() {
            if map.contains_key(&c) {
                if stack.last() == map.get(&c) {
                    stack.pop();
                    continue;
                } else {
                    return false;
                }
            }
            stack.push(c);
        }
        if stack.is_empty() { true } else { false }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(true, Solution::is_valid("()[]{}".to_string()));
        assert_eq!(false, Solution::is_valid("()[]{)(".to_string()));
        assert_eq!(false, Solution::is_valid("(((((((((((((()".to_string()));
    }
}