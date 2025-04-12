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
        
        //Create a dummy head to make swapping easier
        let mut dummy = Box::new(ListNode{val: 0 , next: head});
        let mut prev = &mut dummy;

        while let Some(mut first) = prev.next.take(){

            if let Some(mut second) = first.next.take(){

                let rest = second.next.take();

                first.next = rest;
                second.next = Some(first);
                prev.next = Some(second);

                if let Some(ref mut new_prev) = prev.next{

                    if let Some(ref mut new_new_prev) = new_prev.next{

                        prev = new_new_prev;
                        continue;
                 
                    }

                }
            }
            else{

                prev.next = Some(first);

            }
            break;
        }


        dummy.next


    }
}