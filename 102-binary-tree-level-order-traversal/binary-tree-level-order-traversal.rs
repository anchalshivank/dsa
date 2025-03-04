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
use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        
        //let's use a vecdeque

        let mut dq = VecDeque::new();

        let mut curr = root.clone();

        dq.push_back(curr);
        let mut result = Vec::new();
        
        loop {

            let size = dq.len();
            if size == 0{
                break;
            }
            let mut curr_level = Vec::new();
            for _ in 0..size{

                if let Some(Some(node)) = dq.pop_front(){

                    let node_borrow = node.borrow();
                    curr_level.push(node_borrow.val);

                    if node_borrow.left.is_some(){
                        dq.push_back(node_borrow.left.clone());
                    }
                    if node_borrow.right.is_some(){
                        dq.push_back(node_borrow.right.clone());
                    }


                }

            }

            if curr_level.len() > 0{

            result.push(curr_level);

            }
        }

        result       

    }
}