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
use std::collections::BinaryHeap;


impl PartialOrd for ListNode{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering>{

        Some(self.cmp(other))

    }
}

impl Ord for ListNode{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering{
        other.val.cmp(&self.val)
    }
}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        
        let mut bh = BinaryHeap::new();

        for list in lists{
            bh.push(list);
        }

        let mut result = Some(Box::new(ListNode::new(0)));
        let mut curr = &mut result;

        while let Some(Some(node)) = bh.pop(){

            let new_node = Some(Box::new(ListNode::new(node.val)));

            if let Some(next_node) = node.next{

                bh.push(Some(next_node));

            }

            if let Some(ref mut c) = curr{

                c.next = new_node;
                curr = &mut c.next;

            }

        }

        result.unwrap().next


    }
}