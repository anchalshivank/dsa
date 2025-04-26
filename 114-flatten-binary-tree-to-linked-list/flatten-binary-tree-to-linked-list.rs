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
        
        if root.is_none(){
            return;
        }

        let mut stack = Vec::new();

        stack.push(root.clone());

        // stack.push(root.clone());


        while let Some(Some(mut curr_rc)) = stack.pop(){
            
            let mut curr = curr_rc.borrow_mut();

            

            if curr.right.is_some(){

                stack.push(curr.right.take());

            }

            if curr.left.is_some(){
                stack.push(curr.left.take());
            }

            curr.left = None;

            if let Some(Some(next)) = stack.last(){

                curr.right = Some(next.clone());

            }



        }

    }
}