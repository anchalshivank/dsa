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
    pub fn merge_two_lists(mut list1: Option<Box<ListNode>>, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {


        let mut l1 = &list1;
        let mut l2 = &list2;

        let mut dummy = Some(Box::new(ListNode::new(0)));
        let mut curr = &mut dummy;
        while let (Some(ref p1), Some(ref p2)) = (l1, l2){

            // println!("{:?} {:?}", p1, p2);
            let val = if p1.val < p2.val{
                
                l1 = & p1.next;
                p1.val
            }else{
                l2 = & p2.next;
                p2.val
            };

            let node = ListNode::new(val);

            if let Some(ref mut c) = curr{

                c.next = Some(Box::new(node));
                curr = &mut c.next;

            }
        
        }

        // println!("{:?}", dummy); 

        match (l1, l2){
            (Some(l), None) | (None, Some(l)) => {

                if let Some(ref mut c) = curr{
                    c.next = Some(l.clone());
                }

            },
            _ => {}
        }

        // println!("{:?} {:?}", l1, l2);

        if let Some(next) = dummy{
            next.next
        }else{
            None
        }

        
    }
}