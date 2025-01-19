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
    pub fn is_balanced(root: A) -> bool {
        
        fn height(node: A) -> (i32,bool){

            match node{
                None=> (0, true),
                Some(n) => {
                    let n_b = n.borrow();
                    let (lh, lb) = height(n.borrow().left.clone());
                    if !lb{
                        return (0, false);
                    }
                    let (rh, rb) = height(n.borrow().right.clone());
                    if !rb{
                        return (0, false);
                    }

                    if (lh - rh ).abs()> 1{
                        return (0, false);
                    }
                    (1+ lh.max(rh), true)

                }
            }

        }

        height(root).1

    }
}