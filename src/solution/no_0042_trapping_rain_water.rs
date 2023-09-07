// 42. Trapping Rain Water -- HARD

// Given n non-negative integers representing an elevation map where the width of each bar is 1, compute how much water it can trap after raining.

// Example 1:

// ()[picture]

// Input: height = [0,1,0,2,1,0,1,3,2,1,2,1]
// Output: 6
// Explanation: The above elevation map (black section) is represented by array [0,1,0,2,1,0,1,3,2,1,2,1]. In this case, 6 units of rain water (blue section) are being trapped.
// Example 2:

// Input: height = [4,2,0,3,2,5]
// Output: 9

// Constraints:

// n == height.length
// 1 <= n <= 2 * 10^4
// 0 <= height[i] <= 10^5

pub struct Solution {}

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let (mut left_max, mut right_max) = (0, 0);
        let (mut left_index, mut right_index) = (0, height.len() - 1);
        let mut area = 0;
        while left_index < right_index {
            // choose the point whic height is shorter  to move
            if height[left_index] <= height[right_index] {
                if height[left_index] <= left_max {
                    // when the get a shorter height , it can trap the water
                    area += left_max - height[left_index];
                } else {
                    left_max = height[left_index];
                }
                left_index += 1;
            } else {
                if height[right_index] < right_max {
                    area += right_max - height[right_index];
                } else {
                    right_max = height[right_index];
                }
                right_index -= 1;
            }
        }
        area
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trap() {
        assert_eq!(Solution::trap(vec![4, 2, 0, 3, 2, 5]), 9);
        assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    }
}
