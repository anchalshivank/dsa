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
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut prev = &mut dummy;

        while let Some(mut curr) = prev.as_mut().and_then(|node| node.next.take()){

            if let Some(mut next) = curr.next.take(){

                curr.next = next.next.take();
                next.next = Some(curr);
                prev.as_mut().unwrap().next = Some(next);
                prev = &mut prev.as_mut().unwrap().next.as_mut().unwrap().next;

            }else{
                prev.as_mut().unwrap().next = Some(curr);
                break;
            }


        }



        dummy.unwrap().next
    }
}