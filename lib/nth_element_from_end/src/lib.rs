// Definition for singly-linked list.
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

impl ListNode {
    /*
        First create a head mutable replica since its not mutable as a parm
        then two separate variables (tail, follow) will start at the beginig of the list
        Tail will advance n spaces ahead of follow, so that when tail gets to the end of the list
        follow will be infront of the node we need to delete.
        Move tail and follow down the list until tail is at the last node
        then update follow (its a mutable borrow of mutable head) to skip the next node.
        Replica will have a new value on the head, this keeps follow infront of n.
    */
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        dummy.next = head;
        let mut replica_head = Box::new(dummy);

        let mut tail = replica_head.clone();
        let mut follow = replica_head.as_mut();

        for _ in 0..n {
            tail = tail.next.unwrap();
        }

        while tail.next.is_some() {
            tail = tail.next.unwrap();
            follow = follow.next.as_mut().unwrap();
        }

        let next = follow.next.as_mut().unwrap();
        follow.next = next.next.clone();

        // return our replica, skipping the first dummy value
        replica_head.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_remove_second_to_last_element() {
        let input_fith_node = ListNode {
            next: None,
            val: 5
        };
        let input_fourth_node = ListNode {
            next: Some(Box::new(input_fith_node)),
            val: 4
        };
        let input_third_node = ListNode {
            next: Some(Box::new(input_fourth_node)),
            val: 3
        };
        let input_second_node = ListNode {
            next: Some(Box::new(input_third_node)),
            val: 2
        };
        let input_first_node = ListNode {
            next: Some(Box::new(input_second_node)),
            val: 1
        };
        let n = 2;
        let expected_fourth_node = ListNode {
            next: None,
            val: 5
        };
        let expected_third_node = ListNode {
            next: Some(Box::new(expected_fourth_node)),
            val: 3
        };
        let expected_second_node = ListNode {
            next: Some(Box::new(expected_third_node)),
            val: 2
        };
        let expected_first_node = ListNode {
            next: Some(Box::new(expected_second_node)),
            val: 1
        };
        assert_eq!(ListNode::remove_nth_from_end(Some(Box::new(input_first_node)), n), Some(Box::new(expected_first_node)));
    }
}
