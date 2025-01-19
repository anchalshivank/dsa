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
        let mut max_height = 0;

        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, height: i32, max_height: &mut i32){

            if let Some(n) = node{

                *max_height = (*max_height).max(height+1);
                let n_b = n.borrow();
                dfs(&n_b.left, height + 1, max_height);
                dfs(&n_b.right, height+1, max_height);

            }
        }
        dfs(&root, 0, &mut max_height);
        max_height
    }
}