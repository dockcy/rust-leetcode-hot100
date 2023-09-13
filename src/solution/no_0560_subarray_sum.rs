// 560. Subarray Sum Equals K
// 中等
// 提示
// Given an array of integers nums and an integer k, return the total number of subarrays whose sum equals to k.

// A subarray is a contiguous non-empty sequence of elements within an array.

// Example 1:

// Input: nums = [1,1,1], k = 2
// Output: 2
// Example 2:

// Input: nums = [1,2,3], k = 3
// Output: 2

// Constraints:

// 1 <= nums.length <= 2 * 10^4
// -1000 <= nums[i] <= 1000
// -10^7 <= k <= 10^7

pub struct Solution {}

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut cnt = 0;
        let mut pre_sum = 0;
        let mut subarray_map = std::collections::HashMap::new();
        subarray_map.insert(0, 1);
        for v in nums {
            pre_sum += v;
            if let Some(nums) = subarray_map.get(&(pre_sum - k)) {
                cnt += nums;
            }
            *subarray_map.entry(pre_sum).or_insert(0) += 1;
        }
        cnt
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![1, 2, 3];
        assert_eq!(2, Solution::subarray_sum(nums, 3));
    }
}
