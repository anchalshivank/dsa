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
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {

        let mut result = Vec::new();
        

        fn inorder(node: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>){

            

            if let Some(node_rc) = node{
                let node_borrow = node_rc.borrow();
                inorder(node_borrow.left.clone(), result);
                
                result.push(node_borrow.val);

                inorder(node_borrow.right.clone(), result);
            }

        }
        inorder(root, &mut result);
        result

        
    }
}