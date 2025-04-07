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
    pub fn range_sum_bst(root: Option<A>, low: i32, high: i32) -> i32 {

        fn dfs(node: Option<A> , low: i32, high: i32) -> i32{
            if let Some(node_rc) = node{

                let node_b = node_rc.borrow();
                let val = node_b.val;

                if val<low{
                    return dfs(node_b.right.clone(), low, high);
                }else if val > high{

                    return dfs(node_b.left.clone(), low, high);

                }else{

                    return val + dfs(node_b.left.clone(), low, high)+ dfs(node_b.right.clone(), low, high);

                }

            }else {
                0
            }
        }

        dfs(root, low, high)


    }
}