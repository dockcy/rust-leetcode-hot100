// 239. Sliding Window Maximum
// 困难
// You are given an array of integers nums, there is a sliding window of size k which is moving from the very left of the array to the very right. You can only see the k numbers in the window. Each time the sliding window moves right by one position.
//
// Return the max sliding window.
//
//
//
// Example 1:
//
// Input: nums = [1,3,-1,-3,5,3,6,7], k = 3
// Output: [3,3,5,5,6,7]
// Explanation:
// Window position                Max
// ---------------               -----
// [1  3  -1] -3  5  3  6  7       3
// 1 [3  -1  -3] 5  3  6  7       3
// 1  3 [-1  -3  5] 3  6  7       5
// 1  3  -1 [-3  5  3] 6  7       5
// 1  3  -1  -3 [5  3  6] 7       6
// 1  3  -1  -3  5 [3  6  7]      7
// Example 2:
//
// Input: nums = [1], k = 1
// Output: [1]
//
//
// Constraints:
//
// 1 <= nums.length <= 10^5
// -104 <= nums[i] <= 10^4
// 1 <= k <= nums.length


pub struct Solution {}

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut heap = std::collections::BinaryHeap::new();
        // initialize the priority queue
        for i in 0..k {
            heap.push((nums[i as usize], i));
        }

        let len: i32 = nums.len() as i32;

        let mut ans = vec![];
        //Obviously, the first answer is the the largest one in the initial queue
        ans.push(heap.peek().unwrap().0);

        // move the window until arrive the end of array
        for i in k..len {
            heap.push((nums[i as usize], i));
            // When the largest one is not in the window , pop it!
            while heap.peek().unwrap().1 <= i - k {
                heap.pop();
            }
            ans.push(heap.peek().unwrap().0);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
        assert_eq!(vec![3, 3, 5, 5, 6, 7], Solution::max_sliding_window(nums, 3));
    }
}

