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

// Assume TreeNode is defined elsewhere
type A = Rc<RefCell<TreeNode>>;

impl Solution {
    pub fn insert_into_max_tree(root: Option<A>, val: i32) -> Option<A> {
        fn helper(node: Option<A>, val: i32) -> Option<A> {
            if let Some(node_rc) = node {
                {
                    let mut node_bm = node_rc.borrow_mut();
                    if node_bm.val > val {
                        node_bm.right = helper(node_bm.right.clone(), val);
                        return Some(node_rc.clone()); // ✅ clone here
                    }
                } // now the borrow is fully dropped

                Some(Rc::new(RefCell::new(TreeNode {
                    val,
                    left: Some(node_rc.clone()), // ✅ clone again
                    right: None,
                })))
            } else {
                Some(Rc::new(RefCell::new(TreeNode::new(val))))
            }
        }


        helper(root, val)
    }
}
