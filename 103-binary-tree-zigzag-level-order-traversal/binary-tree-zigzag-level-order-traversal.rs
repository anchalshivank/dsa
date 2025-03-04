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
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {

        let mut dq = VecDeque::new();

        let mut result = Vec::new();
        let mut curr = root.clone();
        dq.push_back(curr);

        let mut flag = true;
    

        loop {

            let size = dq.len();

            if size == 0 {

                break;

            }
            let mut curr = Vec::new();

            for _ in 0..size{
                
             
                if flag{

                    if let Some(Some(node)) = dq.pop_front(){
                        
                        //when I pop from front I need to push them bank
                        let node_borrow = node.borrow();
                        curr.push(node_borrow.val);

                       
                        if node_borrow.left.is_some(){
                            dq.push_back(node_borrow.left.clone());
                        }
                         if node_borrow.right.is_some(){
                            dq.push_back(node_borrow.right.clone());
                        }

                    }

                }else{

                    if let Some(Some(node)) = dq.pop_back(){

                        let node_borrow = node.borrow();
                        curr.push(node_borrow.val);

                        if node_borrow.right.is_some(){

                            dq.push_front(node_borrow.right.clone());

                        }
                        if node_borrow.left.is_some(){

                            dq.push_front(node_borrow.left.clone());

                        }

                    }
                }
            }
            if curr.len() > 0 {

                result.push(curr);
            }
            flag =!flag;        
        }

        result

    }
}