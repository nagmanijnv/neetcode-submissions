// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//     pub val: i32,
//     pub next: Option<Box<ListNode>>,
// }
//
// impl ListNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         ListNode { next: None, val }
//     }
// }

impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut new_list = None;
        let mut new_iter: &mut Option<Box<ListNode>>;

        let mut iter1 = list1;
        let mut iter2 = list2;

        match (iter1.take(), iter2.take()) {
            (Some(mut node1), Some(mut node2)) => {
                if node1.val <= node2.val {
                    iter1 = node1.next.take();
                    new_list = Some(node1);
                    iter2 = Some(node2);
                } else {
                    iter2 = node2.next.take();
                    new_list = Some(node2);
                    iter1 = Some(node1);
                }
                new_iter = &mut new_list.as_mut().unwrap().next;
            }
            (Some(node1), None) => {
                new_list = Some(node1);
                return new_list;
            }
            (None, Some(node2)) => {
                new_list = Some(node2);
                return new_list;
            }
            _ => {
                return new_list;
            }
        }

        if iter1.is_none() || iter2.is_none() {
                if iter1.is_some() {
                    *new_iter = iter1.take();
                } else {
                    *new_iter = iter2.take();
                }
            }

        while let (Some(mut node1), Some(mut node2)) = (iter1.take(), iter2.take()) {
            if node1.val <= node2.val {
                iter1 = node1.next.take();
                *new_iter = Some(node1);
                iter2 = Some(node2);
            } else {
                iter2 = node2.next.take();
                *new_iter = Some(node2);
                iter1 = Some(node1);
            }
            new_iter = &mut new_iter.as_mut().unwrap().next;

            if iter1.is_none() || iter2.is_none() {
                if iter1.is_some() {
                    *new_iter = iter1.take();
                } else {
                    *new_iter = iter2.take();
                }
            }
        }

        new_list
    }
}
