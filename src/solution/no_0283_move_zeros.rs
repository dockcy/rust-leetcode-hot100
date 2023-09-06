// 283. Move Zeroes --easy
// Given an integer array nums, move all 0's to the end of it while maintaining the relative order of the non-zero elements.

// Note that you must do this in-place without making a copy of the array.

// Example 1:

// Input: nums = [0,1,0,3,12]
// Output: [1,3,12,0,0]
// Example 2:

// Input: nums = [0]
// Output: [0]

// Constraints:

// 1 <= nums.length <= 104
// -231 <= nums[i] <= 231 - 1

// Follow up: Could you minimize the total number of operations done?

pub struct Solution {}

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        if nums.len() == 1 {
            return;
        }

        let mut slow_idx = 0;
        for fast in 0..nums.len(){
            if nums[fast] != 0 {
                nums[slow_idx] = nums[fast];
                slow_idx += 1;
            }
        }
        for i in slow_idx..nums.len(){
            nums[i] = 0;
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn basic_test() {
        let mut nums = vec![0,1,0,3,12];
        let res = vec![1,3,12,0,0];
        Solution::move_zeroes(&mut nums);
        assert_eq!(res,nums);
    }
}
