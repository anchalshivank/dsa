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
    pub fn check_tree(root: A) -> bool {
        
        fn helper(node: A) -> (i32, bool){

            match node{
                None => (0,true),
                Some(n) => {

                    let nb = n.borrow();

                    if nb.left.is_none() && nb.right.is_none(){
                        return (nb.val, true);
                    }

                    let (left_sum , left_valid)  = if nb.left.is_none(){
                        (0, true)
                    }else{
                        helper(nb.left.clone())
                    };

                    if !left_valid{
                        return (0, false);
                    }

                    let (right_sum , right_valid) = if nb.right.is_none(){
                        (0, true)
                    }else{

                        helper(nb.right.clone())

                    };

                    if !right_valid {
                        return (0, false);
                    }

                    if nb.val == left_sum + right_sum {
                        (left_sum+ right_sum + nb.val, true)
                    }else{
                        (0, false)
                    }

                }
            }

        }

        helper(root).1
        

    }
}