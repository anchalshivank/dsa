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
    pub fn merge_trees(root1: Option<A>, root2: Option<A>) -> Option<A> {
        

        fn merge(node1: Option<A>, node2: Option<A>) -> Option<A>{


            //traveerse the same way!

            match (node1, node2){

                (Some(n1_rc), Some(n2_rc)) => {


                    let (mut n1b, mut n2b) = (n1_rc.borrow(), n2_rc.borrow());

                    //let calculate the current value!!!
                    let mut node = TreeNode::new(n1b.val+n2b.val);
                    node.left = merge(n1b.left.clone(), n2b.left.clone());
                    node.right = merge(n1b.right.clone(), n2b.right.clone());
                    Some(Rc::new(RefCell::new(node)))

                }
                (_, Some(n_rc)) | (Some(n_rc), _) => {

                    //so we are getting only the single node
                    //just return that
                    Some(n_rc.clone())

                }
                _ => None

            }

        }

        merge(root1, root2)

    }
}