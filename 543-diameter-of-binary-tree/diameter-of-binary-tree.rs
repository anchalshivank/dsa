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
type A = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn diameter_of_binary_tree(root: A) -> i32 {
        
        fn diameter(node: A) -> (i32, i32){

            match node{
                None => (0,0),
                Some(n) => {

                
                let (ld, ldia) = diameter(n.borrow().left.clone());
                let (rd, rdia) = diameter(n.borrow().right.clone());
                (i32::max(ld, rd) +1, i32::max(ldia, i32::max(rdia, ld+rd)))
                }
            }

        }

        diameter(root).1
    

    }
}