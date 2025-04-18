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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

        let mut dummy = Some(Box::new(ListNode::new(0)));
        let mut curr = &mut dummy;

        let mut p = &head;

        while let Some(ref node) = p{
            
            let n = ListNode::new(node.val);

            if let Some(ref mut c) = curr{
                c.next = Some(Box::new(n));
                curr = &mut c.next;
            }

            //see what is next
            let mut next = &node.next;
            while let Some(ref n_next) = next{

                if n_next.val == node.val{

                    //keep traversing next
                    next = & n_next.next;

                }else{
                    //we need to break
                    break;
                }

            }

            //Not this p pointer is ont hat exact thing
            //Now next is some where where that is new I can just do p = next
            p = next;

        }
        dummy.unwrap().next

        
    }
}