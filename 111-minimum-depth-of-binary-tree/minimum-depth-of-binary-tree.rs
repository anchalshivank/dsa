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
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        
        fn depth(node:Option<Rc<RefCell<TreeNode>>>, min_depth: &mut i32, d: i32) {


            match node{

                Some(node_rc) => {
                    let node_b = node_rc.borrow();


                    if node_b.left.is_none() && node_b.right.is_none(){
                        
                        if *min_depth > d{
                            *min_depth = d;
                        } 



                    }else{

                        depth(node_b.left.clone(), min_depth, d+1);
                        depth(node_b.right.clone(),min_depth, d+1);
                                    
                    }
                }
                None => {}

            }

        }

        let mut d = 1;
        let mut min_depth= i32::MAX;
        depth(root,&mut min_depth , d);
        if min_depth == i32::MAX{
            0
        }else{


        min_depth

        }
    }
}