// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//     pub val: i32,
//     pub next: *mut ListNode,
// }
//
// impl ListNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         ListNode { next: std::ptr::null_mut(), val }
//     }
// }

impl Solution {
    pub fn has_cycle(head: *mut ListNode) -> bool {
        let mut slow = head;
        let mut fast = head;

        while !fast.is_null() {
            unsafe {
                slow = (*slow).next;

                // before moving fast 2 step, check if it's next is not null
                if !(*fast).next.is_null() {
                    fast = (*((*fast).next)).next;
                } else {
                    return false;
                }
            }

            if std::ptr::eq(slow, fast) {
                return true;
            }
        }

        return false;
    }
}
