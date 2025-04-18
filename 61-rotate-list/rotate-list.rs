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
    pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        //first let's calculate the length
        if head.is_none() || k == 0 {
            return head;
        }
        let mut n = 0;

        let mut curr = &head;

        while let Some(node_b) = curr{

            curr = & node_b.next;
            n+=1;


        }   
        let k = k%n;
        if k == 0{
            return head;
        }
        let mut a = n - k;
        let mut k = a;
        // drop(curr);

        let mut p = &mut head;


        println!("{:?}", k );


        while k > 0 {

            if let Some(node_b) = p{

                p = &mut node_b.next;

            }

            k-=1;

        }


        let mut take  = p.take();
        let mut tp = &mut take;
        let mut to_travel = n-a;
        println!("{:?}", tp);
        while to_travel>1{

            if let Some(ref mut node_b) = tp{

                tp = &mut node_b.next;

            }

            to_travel -=1;

        }

        if let Some(ref mut node_b) = tp{
            node_b.next = head;
        }

        take

    }
}