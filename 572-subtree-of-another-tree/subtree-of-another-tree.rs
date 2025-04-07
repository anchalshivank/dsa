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
    pub fn is_subtree(root: Option<A>, sub_root: Option<A>) -> bool {

        //we need to confirm
        //let's traverse the tree

        fn helper(node: Option<A>, sub: Option<A>) -> bool {
    match (node, sub) {
        (Some(node_rc), Some(sub_rc)) => {
            let node_b = node_rc.borrow();
            let sub_b = sub_rc.borrow();

            node_b.val == sub_b.val
                && helper(node_b.left.clone(), sub_b.left.clone())
                && helper(node_b.right.clone(), sub_b.right.clone())
        }
        (None, None) => true,
        _ => false,
    }
}


        fn pre(node: Option<A> , sub: Option<A>) -> bool{
            
            //we need to confirm that sub exist or not!!
            match (node, sub){
                (Some(node_rc), Some(sub_rc)) => {
                    let node_b = node_rc.borrow();
                    
                    let sub_b = sub_rc.borrow();

                    if node_b.val == sub_b.val{

                        //we need to check that this is equal or not!
                        //once we reach here just compare recurssivedly

                        if !helper(Some(node_rc.clone()), Some(sub_rc.clone())){
                            if pre(node_b.left.clone(), Some(sub_rc.clone())){
                            return true;
                        }else{
                            pre(node_b.right.clone(), Some(sub_rc.clone()))
                        }
                        }else{
                            true
                        }

                    }else{
                        //keep traversing!!

                        if pre(node_b.left.clone(), Some(sub_rc.clone())){
                            return true;
                        }else{
                            pre(node_b.right.clone(), Some(sub_rc.clone()))
                        }
                        

                    }

                }
                _ => false
            }


        }


        pre(root, sub_root)


        
    }
}