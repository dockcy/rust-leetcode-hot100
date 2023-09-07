// 15. 3Sum --medium

// Given an integer array nums, return all the triplets [nums[i], nums[j], nums[k]] such that i != j, i != k, and j != k, and nums[i] + nums[j] + nums[k] == 0.

// Notice that the solution set must not contain duplicate triplets.

// Example 1:

// Input: nums = [-1,0,1,2,-1,-4]
// Output: [[-1,-1,2],[-1,0,1]]
// Explanation:
// nums[0] + nums[1] + nums[2] = (-1) + 0 + 1 = 0.
// nums[1] + nums[2] + nums[4] = 0 + 1 + (-1) = 0.
// nums[0] + nums[3] + nums[4] = (-1) + 2 + (-1) = 0.
// The distinct triplets are [-1,0,1] and [-1,-1,2].
// Notice that the order of the output and the order of the triplets does not matter.
// Example 2:

// Input: nums = [0,1,1]
// Output: []
// Explanation: The only possible triplet does not sum up to 0.
// Example 3:

// Input: nums = [0,0,0]
// Output: [[0,0,0]]
// Explanation: The only possible triplet sums up to 0.

// Constraints:

// 3 <= nums.length <= 3000
// -10^5 <= nums[i] <= 10^5

pub struct Solution {}


impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 3 {
            panic!()
        }
        let mut nums = nums;
        nums.sort();
        let target = 0;
        let mut ans = vec![];
        for i in 0..nums.len() - 2 {
            // ensure there's no duplicate answer
            if i >= 1 && nums[i] == nums[i - 1] {
                continue;
            }
            let two_sum = target - nums[i];
            // j can only move to right
            let mut j = i + 1;
            // k can only move to left
            let mut k: usize = nums.len() - 1;
            // j doesn't need to move the array's end
            while j < k {
                // ensure there's no duplicate answer
                if j != i + 1 && nums[j] == nums[j - 1] {
                    j += 1;
                    continue;
                }
                // Pin the j and find another nums[k] from large  to small
                while j < k && nums[j] + nums[k] > two_sum {
                    k -= 1;
                }

                // It means that there's no 'k' to meet the conditions
                if j == k {
                    break;
                }
                // Get the answer
                if nums[j] + nums[k] == two_sum {
                    let arr = vec![nums[i], nums[j], nums[k]];
                    ans.push(arr);
                }
                // less than the two_sum means nums[j] must be larger
                j += 1;
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
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let ans = Solution::three_sum(nums);
        print!("{:?}", ans);
    }
}
