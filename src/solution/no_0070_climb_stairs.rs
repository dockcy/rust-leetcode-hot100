// 70. Climbing Stairs
// 简单
// You are climbing a staircase. It takes n steps to reach the top.
//
// Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?
//
//
//
// Example 1:
//
// Input: n = 2
// Output: 2
// Explanation: There are two ways to climb to the top.
// 1. 1 step + 1 step
// 2. 2 steps
// Example 2:
//
// Input: n = 3
// Output: 3
// Explanation: There are three ways to climb to the top.
// 1. 1 step + 1 step + 1 step
// 2. 1 step + 2 steps
// 3. 2 steps + 1 step
//
//
// Constraints:
//
// 1 <= n <= 45

pub struct Solution {}

impl Solution {
    /**
    Satisfied the formula "g(n) = g(n-1) + g(n-2)" -- Just a Fibonacci sequence
     */
    pub fn climb_stairs(n: i32) -> i32 {
        if n == 1 { return 1; }
        if n == 2 { return 2; }
        let mut last_two = 0;
        let mut last_one = 1;
        let mut ans = 0;
        for _i in 1..=n {
            ans = last_one + last_two;
            last_two = last_one;
            last_one = ans;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, Solution::climb_stairs(3));
        assert_eq!(5, Solution::climb_stairs(4));
    }
}