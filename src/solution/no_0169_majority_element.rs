// 169. Majority Element
// 简单
// Given an array nums of size n, return the majority element.
//
// The majority element is the element that appears more than ⌊n / 2⌋ times. You may assume that the majority element always exists in the array.
//
//
//
// Example 1:
//
// Input: nums = [3,2,3]
// Output: 3
// Example 2:
//
// Input: nums = [2,2,1,1,1,2,2]
// Output: 2
//
//
// Constraints:
//
// n == nums.length
// 1 <= n <= 5 * 10^4
// -10^9 <= nums[i] <= 10^9
//
//
// Follow-up: Could you solve the problem in linear time and in O(1) space?
pub struct Solution {}

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut cnt = 0;
        let mut candidate = 0;
        for x in nums {
            if cnt == 0 {
                candidate = x;
                cnt += 1;
            } else if x == candidate {
                cnt += 1;
            } else {
                cnt -= 1;
            }
        }
        candidate
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]));
        assert_eq!(3, Solution::majority_element(vec![3,3,4]));
        assert_eq!(9, Solution::majority_element(vec![10,9,9,9,10]));
    }
}
