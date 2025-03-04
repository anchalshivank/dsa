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
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        
        match (p,q) {

            (Some(p_rc), Some(q_rc)) => {

                let p_b = p_rc.borrow();
                let q_b = q_rc.borrow();
                if p_b.val == q_b.val{

                    return Self::is_same_tree(p_b.left.clone(), q_b.left.clone()) && Self::is_same_tree(p_b.right.clone(), q_b.right.clone());

                }else{
                    return false;
                }

            },

            (None, Some(_)) | (Some(_), None) => {
                false
            }
            _ => {
                true
            }

        }

    }
}