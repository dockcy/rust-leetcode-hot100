// 118. Pascal's Triangle
// 简单
// Given an integer numRows, return the first numRows of Pascal's triangle.
//
// In Pascal's triangle, each number is the sum of the two numbers directly above it as shown:
//
//
//
//
// Example 1:
//
// Input: numRows = 5
// Output: [[1],[1,1],[1,2,1],[1,3,3,1],[1,4,6,4,1]]
// Example 2:
//
// Input: numRows = 1
// Output: [[1]]
//
//
// Constraints:
//
// 1 <= numRows <= 30

pub struct Solution {}

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        ans.push(vec![1]);
        if num_rows == 1 { return ans; };
        ans.push(vec![1, 1]);
        if num_rows == 2 { return ans; };

        for i in 3..=num_rows as usize {
            let mut tmp = vec![1];
            for m in 0..ans[i - 2].len()-1 {
                tmp.push(ans[i - 2][m] + ans[i - 2][m+1] );
            }
            tmp.push(1);
            ans.push(tmp);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut res = vec![];
        res.push(vec![1]);
        res.push(vec![1,1]);
        res.push(vec![1,2,1]);
        res.push(vec![1,3,3,1]);
        res.push(vec![1,4,6,4,1]);

        assert_eq!(res,Solution::generate(5));
    }
}