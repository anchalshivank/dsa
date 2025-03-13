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
use std::collections::VecDeque;
impl Solution {
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {

        
        let mut q = VecDeque::new();

        let mut result = Vec::new();

        q.push_back(root.clone());

        loop{


            //get q length

            let len = q.len();

            if len == 0{
                break;
            }
            let mut curr_level = Vec::new();
            for _ in 0..len{

                if let Some(Some(curr_rc)) = q.pop_front(){

                    //we have the value

                    let curr_b = curr_rc.borrow();
                    curr_level.push(curr_b.val);

                    if curr_b.left.is_some(){

                                //we shall push the next node
                                q.push_back(curr_b.left.clone());

                    }

                    if curr_b.right.is_some(){

                            q.push_back(curr_b.right.clone());

                    }


                }

            }

            if curr_level.len() > 0{

            result.push(curr_level);
            }


        }

        //now we need to reverse th vector 
        result.reverse();

        result





        
    }
}