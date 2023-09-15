// 206. Reverse Linked List
// 简单
// Given the head of a singly linked list, reverse the list, and return the reversed list.
//
// Example 1:
//
//
// Input: head = [1,2,3,4,5]
// Output: [5,4,3,2,1]
// Example 2:
//
//
// Input: head = [1,2]
// Output: [2,1]
// Example 3:
//
// Input: head = []
// Output: []
//
// Constraints:
//
// The number of nodes in the list is the range [0, 5000].
// -5000 <= Node.val <= 5000


#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            val,
            next: None,
        }
    }
}

pub struct Solution {}

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut pre = None;
        let mut head = head;
        while let Some(mut node) = head {
            head = node.next.take();
            node.next = pre;
            pre = Some(node);
        }
        pre
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_list() {
        let mut head = ListNode::new(1);
        head.next = Some(Box::new(ListNode::new(2)));
        head.next.as_mut().unwrap().next = Some(Box::new(ListNode::new(3)));


        let new_head = Solution::reverse_list(Some(Box::new(head)));

        assert_eq!(new_head.as_ref().unwrap().val, 3);
        assert_eq!(new_head.as_ref().unwrap().next.as_ref().unwrap().val, 2);
        assert_eq!(new_head.as_ref().unwrap().next.as_ref().unwrap().next.as_ref().unwrap().val, 1);
    }
}

