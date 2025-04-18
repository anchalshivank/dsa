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
    pub fn add_two_numbers(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        
        //this is that we are transferring ownership
        //But this p1 shall be reference

        let mut head = Box::new(ListNode::new(0));
        let mut current = &mut head;

        let mut carry = 0;

        while (l1 != None || l2 != None || carry != 0){

            let val = match l1{
                Some(n) => {l1 = n.next; n.val},
                None => 0
            }+ match l2{
                Some(n) => {l2 = n.next; n.val},
                None => 0
            }+carry;

            carry = val/10;
            current.next = Some(Box::new(ListNode::new(val%10)));
            current = current.next.as_mut().unwrap();

        }

        head.next

    }
}