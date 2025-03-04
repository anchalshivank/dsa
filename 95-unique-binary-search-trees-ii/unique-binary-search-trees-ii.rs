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
        
        fn helper(start: i32, end: i32) -> Vec<A> {

            let mut result = Vec::new();

            if start> end{
                result.push(None);
                return result;
            }

            for val in start..=end{

                

                let lefts = helper(start, val -1);
                let rights = helper(val+1, end);

                for left_node in &lefts{


                    for right_node in &rights{

                        let mut curr_node = TreeNode::new(val);
                        curr_node.left = left_node.clone();
                        curr_node.right = right_node.clone();
                        let to_push = Some(Rc::new(RefCell::new(curr_node))); 
                        result.push(to_push);



                    }

                } 



            }
            result
        }

        helper(1, n)


    }
}