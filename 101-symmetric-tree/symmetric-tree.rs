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
type A = Rc<RefCell<TreeNode>>;
impl Solution {
    pub fn is_symmetric(root: Option<A>) -> bool {

        fn dfs(node_a: Option<A>, node_b: Option<A>) -> bool{

            match (node_a, node_b){

                (Some(node_a_rc), Some(node_b_rc)) => {
                    let (mut a, mut b) = (node_a_rc.borrow(), node_b_rc.borrow());
                    if a.val == b.val{

                        dfs(a.left.clone(), b.right.clone()) && dfs(a.right.clone(), b.left.clone())

                    }else{
                        return false;
                    }

                }
                (None, None) => true,
                _ => false


            }

        }

        if let Some(r_rc) = root{

            let r_b = r_rc.borrow();
            return dfs(r_b.left.clone(), r_b.right.clone());

        }
        true
                
    }
}