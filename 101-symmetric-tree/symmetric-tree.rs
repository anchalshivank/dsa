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
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {

            fn dfs(p1: &Option<Rc<RefCell<TreeNode>>>, p2: &Option<Rc<RefCell<TreeNode>>>) -> bool{

                if p1.is_none() && p2.is_none(){
                    true
                }else if let (Some(p1_node), Some(p2_node)) = (p1, p2){

                    let p1b = p1_node.borrow();
                    let p2b = p2_node.borrow();

                    if p1b.val != p2b.val{
                        false
                    }

                    else {

                        dfs(&p1b.left, &p2b.right) && dfs(&p1b.right, &p2b.left)
                    }


                }else{
                    false
                }

            }

            if let Some(ref root_rc) = root{

                let root_b = root_rc.borrow();
                dfs(&root_b.left, &root_b.right)

            }else{
                true
            }

        
    }
}