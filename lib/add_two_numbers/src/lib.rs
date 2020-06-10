/*
You are given two non-empty linked lists representing two non-negative integers. The digits are stored in reverse order and each of their nodes contain a single digit. Add the two numbers and return it as a linked list.

You may assume the two numbers do not contain any leading zero, except the number 0 itself.

Example:

Input: (2 -> 4 -> 3) + (5 -> 6 -> 4)
Output: 7 -> 0 -> 8
Explanation: 342 + 465 = 807.

*/

//Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
struct Solution {}
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        // for every node value in each list add them together
        // if the sum is greater than 10, add 1 to the next sum
        // if either value doesn't exist, make it zero
        let mut sum = 0;
        let mut modified_node: Option<Box<ListNode>> = None;
        match (l1, l2) {
            (Some(x), Some(y)) => {
                //handle remainder
                sum = x.val + y.val;
                if sum >= 10 {
                    // update the next l1 value to add one for the remainder
                    match x.next {
                        Some(z) => {
                            modified_node = Some(Box::new(ListNode {
                                next: z.next,
                                val: z.val + 1,
                            }))
                        }
                        None => modified_node = Some(Box::new(ListNode { next: None, val: 1 })),
                    }
                    return Some(Box::new(ListNode {
                        next: Solution::add_two_numbers(modified_node, y.next),
                        val: sum % 10,
                    }));
                } else {
                    return Some(Box::new(ListNode {
                        next: Solution::add_two_numbers(x.next, y.next),
                        val: sum,
                    }));
                }
            }
            (Some(x), None) => {
                if x.val >= 10 {
                    return Some(Box::new(ListNode {
                        next: Solution::add_two_numbers(x.next, Some(Box::new(ListNode::new(1)))),
                        val: x.val % 10,
                    }));
                }
                return Some(Box::new(ListNode {
                    next: Solution::add_two_numbers(x.next, None),
                    val: x.val,
                }));
            }
            (None, Some(y)) => {
                return Some(Box::new(ListNode {
                    next: Solution::add_two_numbers(None, y.next),
                    val: y.val,
                }))
            }
            (None, None) => return None,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(true, true);
    }
}
