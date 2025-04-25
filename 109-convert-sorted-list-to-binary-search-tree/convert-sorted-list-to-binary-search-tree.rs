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
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {

        //if len is 3 , mid shall be 1 
        // if len is 4 than root shall be 2

        fn len(mut head: &Option<Box<ListNode>>) -> i32{

            let mut i = 0;
            while let Some(curr) = head{
                head = &curr.next;
                i+=1;
            }

            i

        }


        fn helper(mut head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>>{
            
            // println!("{:?}", head);
            if head.is_none(){
                return None;
            }
            let n = len(&head);
            // println!("{:?}", n);
            if n == 0{

                return None;                

            }else if n == 1{
                    if let Some(s) = head{

                        let node = Some(Rc::new(RefCell::new(TreeNode::new(s.val))));
                        return node;
                    }else{

                        return None;

                    }

            }
            let t = n/2;





            let mut slow = &mut head;
            let mut i = 1;
            let mut rest = None;
            // we need that node

            let mut node = None;
            while let Some(ref mut s) = slow{

                if i == t{
                    
                    rest = s.next.take();
                    break;
                }
                i+=1;
                slow = &mut s.next;
                

            }

            let mut slow = &mut rest;
            if let Some(ref mut s) = slow{
                node = Some(Rc::new(RefCell::new(TreeNode::new(s.val))));
                rest = s.next.take();

            
            }

            if let Some(ref mut node_rc) = node{
                let mut node_bm = node_rc.borrow_mut();

                // println!("{:?} -->> {:?} <<--- {:?}", head, node_bm.val, rest);
                node_bm.left = helper(head);
                node_bm.right = helper(rest);
            }

            node

    

        }
        

        helper(head)

    }
}