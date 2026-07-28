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
        let mut new_iter = &mut new_list;

        let mut iter1 = list1;
        let mut iter2 = list2;

        loop {
            if iter1.is_some() && iter2.is_some() {
                if iter1.as_ref().unwrap().val <= iter2.as_ref().unwrap().val {
                    let mut node1 = iter1.take().unwrap();
                    iter1 = node1.next.take();
                    *new_iter = Some(node1);
                } else {
                    let mut node2 = iter2.take().unwrap();
                    iter2 = node2.next.take();
                    *new_iter = Some(node2);
                }
                new_iter = &mut new_iter.as_mut().unwrap().next;
            } else if iter1.is_some() && iter2.is_none() {
                *new_iter = iter1.take();
                break;

            } else if iter1.is_none() && iter2.is_some() {
                *new_iter = iter2.take();
                break;
            } else {
                break;
            }
        }
        new_list
    }
}
