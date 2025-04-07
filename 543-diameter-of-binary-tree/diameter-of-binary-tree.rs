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
        
        fn diameter(node:A) -> (i32, i32){

            if let Some(node_rc) = node{

                //one way is that we return the max ehight .. and max dia
                let node_b = node_rc.borrow();

                let (lh, ld) = diameter(node_b.left.clone());
                let (rh, rd) = diameter(node_b.right.clone());

                let curr_dia = lh+rh;

                let max_dia = curr_dia.max(ld.max(rd));
                let max_h = lh.max(rh) + 1;
                return (max_h, max_dia);
                


            }return {
                (0,0)
            }

        }

        diameter(root).1
    

    }

}