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
        
        let mut ml = 0;
        let mut mr = 0;

        fn height(node: A) -> i32{
            if let Some(n) = node{
                let nb = n.borrow();
                
                let left_h = height(nb.left.clone());
                let right_h = height(nb.right.clone());
                return 1+left_h.max(right_h)

            }else{
                0
            }
        }
        
        fn diameter(node: A) -> i32{

            if let Some(n) = node{
                let nb = n.borrow();
                let left_h = height(nb.left.clone());
                let right_h = height(nb.right.clone());
                
                let left_dia = diameter(nb.left.clone());
                let right_dia = diameter(nb.right.clone());

                return i32::max(left_h + right_h, i32::max(left_dia, right_dia));

            }else{
                0
            }

        }

        diameter(root)
    

    }
}