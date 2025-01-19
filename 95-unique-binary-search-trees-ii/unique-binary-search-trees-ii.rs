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
    pub fn generate_trees(n: i32) -> Vec<A> {
        

        fn helper(start: i32, end: i32 ) -> Vec<A>{
            let mut result = Vec::new();
            if start > end {
                result.push(None);
                return result;
            }

                for i in start..=end{

                   
                    let left_nodes= helper(start, i -1);
                    let right_nodes = helper(i+1, end);
                    for ln in &left_nodes{
                        for rn in &right_nodes{
                            
                            let mut node = TreeNode::new(i);
                            node.left = ln.clone();
                            node.right = rn.clone();
                            result.push(Some(Rc::new(RefCell::new(node))));
                            
                        }
                    }
                }
                result
        

        }

        helper(1, n)

    }
}