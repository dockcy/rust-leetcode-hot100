// Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.

// You may assume that each input would have exactly one solution, and you may not use the same element twice.

// You can return the answer in any order.

// Example 1:

// Input: nums = [2,7,11,15], target = 9
// Output: [0,1]
// Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
// Example 2:

// Input: nums = [3,2,4], target = 6
// Output: [1,2]
// Example 3:

// Input: nums = [3,3], target = 6
// Output: [0,1]

// Constraints:

// 2 <= nums.length <= 104
// -109 <= nums[i] <= 109
// -109 <= target <= 109
// Only one valid answer exists.

// Follow-up: Can you come up with an algorithm that is less than O(n2) time complexity?

use std::collections::HashMap;
pub struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() <= 1 {
            panic!();
        }
        let mut index_map = HashMap::new();
        for i in 0..nums.len() {
            if let Some(v) = index_map.get(&(target - nums[i])) {
                return vec![i as i32, *v as i32];
            }
            index_map.insert(nums[i], i);
        }
        panic!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn basic_test() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        assert_eq!(vec![1, 0], Solution::two_sum(nums, target));
    }
}
