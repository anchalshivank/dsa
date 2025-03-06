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
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        
        fn helper(node:Option<Rc<RefCell<TreeNode>>>, curr_sum: i32, target_sum: i32) -> bool {

            match node{

                Some(node_rc) => {
                    let node_b = node_rc.borrow();
                    let val = node_b.val;
                    
                    if curr_sum + val == target_sum && node_b.left.is_none() && node_b.right.is_none(){
                        return true;
                    }else{
                        let left  = helper(node_b.left.clone(), curr_sum + val, target_sum);
                        if left{
                            return true;
                        }
                        let right = helper(node_b.right.clone(), curr_sum+ val, target_sum);
                        right
                    }

                }
                None => {
                    false
                }

            }

        }

        helper(root, 0, target_sum)

    }
}