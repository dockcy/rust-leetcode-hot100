// 136. Single Number
// 简单
// Given a non-empty array of integers nums, every element appears twice except for one. Find that single one.
//
// You must implement a solution with a linear runtime complexity and use only constant extra space.
//
//
//
// Example 1:
//
// Input: nums = [2,2,1]
// Output: 1
// Example 2:
//
// Input: nums = [4,1,2,1,2]
// Output: 4
// Example 3:
//
// Input: nums = [1]
// Output: 1
//
//
// Constraints:
//
// 1 <= nums.length <= 3 * 10^4
// -3 * 10^4 <= nums[i] <= 3 * 10^4
// Each element in the array appears twice except for one element which appears only once.

pub struct Solution {}

impl Solution {
    // Any XOR operation between a number 'a' and 0  is the original number 'a' : a ⊕ 0 = a
    // Any XOR operation between a number 'a' and 'a'   is the original number 'a' : a ⊕ a = 0
    // a ⊕ b ⊕ c  = (b ⊕ c )  ⊕ a
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.iter().fold(0, |acc, &x| acc ^ x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(4, Solution::single_number(vec![4, 1, 2, 1, 2]));
    }
}