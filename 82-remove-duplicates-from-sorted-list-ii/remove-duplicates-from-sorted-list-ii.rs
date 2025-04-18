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
        let mut tail = &mut dummy;
        let mut p = &head;

        while let Some(ref curr) = p{
            let mut is_duplicate = false;

            while let Some(ref next) = &curr.next{
                if next.val == curr.val{

                    is_duplicate = true;
                    p = &curr.next;
                    break;
                }else{
                    break;
                }

            }

            if is_duplicate {

                let val = curr.val;
                while let Some(n) = p{
                    if n.val == val{
                        p = &n.next;
                    }else{
                        break;
                    }
                }

            }else{
                if let Some(tail_node) = tail{

                    tail_node.next = Some(Box::new(ListNode::new(curr.val)));
                    tail = &mut tail_node.next;

                }

                p = & curr.next;

            }

        }
        dummy.unwrap().next

    }
}