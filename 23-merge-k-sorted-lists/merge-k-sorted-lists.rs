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
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        
        //ok see I can have a res linked list and merge them 
        //time complexity shall remain same even incase of divide and conquer

        //let's create a simple function to merge a list


        fn merge(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>>{

            let mut l1 = &list1;
            let mut l2 = &list2;

            let mut dummy = Some(Box::new(ListNode::new(0)));
            let mut curr = &mut dummy;

            while let (Some(p1), Some(p2)) = (l1, l2){

                let node = if p1.val< p2.val{
                    l1 = & p1.next;
                    Some(Box::new(ListNode::new(p1.val)))

                }else{
                    l2 = &p2.next;
                    Some(Box::new(ListNode::new(p2.val)))

                };

                if let Some(ref mut c) = curr{

                    c.next = node;
                    curr = &mut c.next;

                }

            }

            match (l1, l2){
                (Some(p), None) | (None, Some(p)) => {

                    if let Some(ref mut c) = curr{


                        c.next = Some(p.clone());

                    }
                    

                }
                _ => {}
            }

            if let Some(d) = dummy{
                d.next
            }else{
                None
            }
        
        }

        let mut res = None;

        for list in lists{

            res = merge(list, res);

        }

        res

    }
}