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
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        //this is flattened tree node
        
        //we shall use stack for having 
        if root.is_none(){
            return;
        }

        let mut stack = Vec::new();
        if let Some(r) = root{
            stack.push(r.clone());
        }
        let mut head = Rc::new(RefCell::new(TreeNode::new(0)));
        while let Some(node) = stack.pop(){

            let mut node_bm = node.borrow_mut();

            if let Some(right) = node_bm.right.take(){
                stack.push(right);
            }

            if let Some(left) = node_bm.left.take(){
                stack.push(left);
            }

            drop(node_bm);

            head.borrow_mut().right = Some(node.clone());
            head = node;

        }
        
    }
}