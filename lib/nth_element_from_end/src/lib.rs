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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        None
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
