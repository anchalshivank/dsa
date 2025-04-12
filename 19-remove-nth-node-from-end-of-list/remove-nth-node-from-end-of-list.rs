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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {

        let mut p1 = &head;
        let mut p2 = &head;

        let mut j = 0;


        while j <n{
            if let Some(ref node) = p1{
                p1 = &node.next;
            }
            j+=1;
        }

        // println!("{:?}", p1);

        let mut ans = Some(Box::new(ListNode::new(0)));
        let mut curr = &mut ans;

        while let Some(ref fast) = p1{

            p1 = &fast.next;
            if let Some(ref slow) = p2{
                let node = Box::new(ListNode::new(slow.val));
                if let Some(ref mut c) = curr{
                    c.next = Some(node);
                    curr = &mut c.next;
                }
                p2 = &slow.next;
            }

        }

        // println!("{:?}", p2);


        //Now I want to convert this p2 to mutable reference!!!
        //and drop p1 to keep rust ownership and borrowing rules How to do that??!

        //now we shall skip next for slow pointer!
    
        
        if let Some(ref node) = p2{
            p2 = &node.next;
        }

        // println!("{:?}", p2);

        // while let Some(ref slow) = p2{
            
        //     let node = Box::new(ListNode::new(slow.val));

        //     println!("{:?}", slow);

        //     if let Some(ref mut c) = curr{
        //         c.next = Some(node);
        //     }
        //     p2 = &slow.next;

        // }
        
        while let Some(ref slow) = p2{
            let node = ListNode::new(slow.val);
            if let Some(ref mut c) = curr{
                c.next = Some(Box::new(node));
                curr = &mut c.next;
            }
            p2 = &slow.next;



        }

        // println!("{:?} -------- ", p2);
        


        

        
        
        ans.unwrap().next

    }
}