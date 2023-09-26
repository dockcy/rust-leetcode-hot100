// 234. Palindrome Linked List
// 简单
// Given the head of a singly linked list, return true if it is a
// palindrome
// or false otherwise.
//
//
//
// Example 1:
//
//
// Input: head = [1,2,2,1]
// Output: true
// Example 2:
//
//
// Input: head = [1,2]
// Output: false
//
//

// Constraints:
//
// The number of nodes in the list is in the range [1, 10^5].
// 0 <= Node.val <= 9



// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}


pub struct Solution {}

impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut vec = vec![];
        let mut head = head;
        while let Some(v) = &head {
            vec.push(v.val);
            head = head.unwrap().next;
        }
        let len = vec.len();
        for i in 0..len / 2 {
            if vec[i] != vec[len - i - 1] { return false; }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let list = ListNode {
            next: Some(Box::new(ListNode {
                next: Some(Box::new(ListNode {
                    next: None,
                    val: 1,
                })),
                val: 0,
            })),
            val: 1,
        };
        assert_eq!(true, Solution::is_palindrome(Some(Box::new(list))));
    }
}