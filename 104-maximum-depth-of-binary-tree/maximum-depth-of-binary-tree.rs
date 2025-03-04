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
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        
        fn depth(node: Option<Rc<RefCell<TreeNode>>>, current: i32) -> i32 {

            if let Some(node_rc) = node{
                
                let nb = node_rc.borrow();
                let lh = depth(nb.left.clone(), current+1);
                let rh = depth(nb.right.clone(), current+1);
                let max_h = lh.max(rh);
                max_h

            }else{
                current
            }

        }

        depth(root, 0)

    }
}