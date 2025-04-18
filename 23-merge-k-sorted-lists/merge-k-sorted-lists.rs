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

impl Ord for ListNode{

    fn cmp(&self, other: &Self) -> std::cmp::Ordering{

        other.val.cmp(&self.val)

    }

}

impl PartialOrd for ListNode{

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering>{
        Some(self.cmp(&other))
    }

}

use std::collections::BinaryHeap;
impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {

        //we will use biary heap
        let mut pq = BinaryHeap::new();

        //the top  element shall be the minium
        for list in lists{
            pq.push(list);
        }
        
        let mut dummy = Some(Box::new(ListNode::new(1)));
        let mut curr = &mut dummy;

        while let Some(Some(mut node)) = pq.pop(){

            let new_node = Some(Box::new(ListNode::new(node.val)));

  
            if let Some(next_node) = node.next{
                pq.push(Some(next_node));
            }


            if let Some(ref mut curr_b) = curr{
                curr_b.next = new_node;
                curr = &mut curr_b.next;
            }


            

        }

        dummy.unwrap().next
        
    }
}