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
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        
        fn dfs(p: &Option<Rc<RefCell<TreeNode>>>, q: &Option<Rc<RefCell<TreeNode>>>) -> bool{
            
            if p.is_none() && q.is_none(){
                return true;
            }
            else if let (Some(p_node), Some(q_node))  = (p, q){
                let v1 = p_node.borrow();
                let v2 = q_node.borrow();

                if v1.val != v2.val {

                    return false;


                }

                return dfs(&v1.left, &v2.left) && dfs(&v1.right, &v2.right);

            }
            false

        }
        dfs(&p, &q)

    }
}