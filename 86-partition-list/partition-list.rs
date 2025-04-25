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
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {

        let mut small = Some(Box::new(ListNode::new(0)));
        let mut sp = &mut small;

        let mut big   = Some(Box::new(ListNode::new(0)));
        let mut bp = &mut big;

        let mut curr = &head;

        while let Some(ref curr_node) = curr{
            let val = curr_node.val;
            let node = ListNode::new(val);
            if val < x{

                if let Some(ref mut s) = sp{

                    s.next = Some(Box::new(node));
                    sp = &mut s.next;

                }

            }else{

                if let Some(ref mut b) = bp{

                    b.next = Some(Box::new(node));
                    bp = &mut b.next;
                
                }

            }
            curr = &curr_node.next;

        }

        if let Some(mut b) = big{
            let mut take = b.next.take();
            if let Some(ref mut s) = sp{
                s.next = take;
            }
        }

        small.unwrap().next


        
    }
}