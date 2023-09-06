// 128. Longest Consecutive Sequence -- medium
// Given an unsorted array of integers nums, return the length of the longest consecutive elements sequence.

// You must write an algorithm that runs in O(n) time.

// Example 1:

// Input: nums = [100,4,200,1,3,2]
// Output: 4
// Explanation: The longest consecutive elements sequence is [1, 2, 3, 4]. Therefore its length is 4.
// Example 2:

// Input: nums = [0,3,7,2,5,8,4,6,0,1]
// Output: 9

// Constraints:

// 0 <= nums.length <= 105
// -109 <= nums[i] <= 109

use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.len() <= 1 {
            return nums.len() as i32;
        }
        let nums = nums.into_iter().collect::<HashSet<i32>>();
        let mut max_len = 1;
        for &v in nums.iter() {
            if nums.contains(&(v - 1)) {
                continue;
            }

            let mut len = 1;
            while nums.contains(&(v + len)) {
                len += 1;
            }
            max_len = i32::max(max_len, len);
        }
        max_len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_consecutive() {
        assert_eq!(Solution::longest_consecutive(vec![]), 0);

        assert_eq!(Solution::longest_consecutive(vec![1]), 1);

        assert_eq!(Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);

        assert_eq!(
            Solution::longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]),
            9
        );

        assert_eq!(Solution::longest_consecutive(vec![1, 2, 0, 1]), 3);
    }
}
