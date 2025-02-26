// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn swap_nodes(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut n = 0;
    let mut node = &head;
    while node.is_some() {
        n += 1;
        node = &node.as_ref().unwrap().next;
    }
    let a = k.min(n - k + 1);
    let b = n - a + 1;
    
    let mut pointer = &mut head;
    for _ in 1..a {
        pointer = &mut pointer.as_mut().unwrap().next;
    }
    let a_val = pointer.as_mut().unwrap().val;
    for _ in a..b {
        pointer = &mut pointer.as_mut().unwrap().next;
    }
    let b_val = pointer.as_mut().unwrap().val;
    pointer.as_mut().unwrap().val = a_val;

    pointer = &mut head;
    for _ in 1..a {
        pointer = &mut pointer.as_mut().unwrap().next;
    }
    pointer.as_mut().unwrap().val = b_val;
    head
}
}