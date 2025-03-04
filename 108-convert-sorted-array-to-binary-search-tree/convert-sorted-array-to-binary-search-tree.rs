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
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        
        fn helper(nums: &Vec<i32> , start: i32, end: i32) -> Option<Rc<RefCell<TreeNode>>> {

            if start > end {
                return None;
            }

            let mid = start + (end - start)/2;

            let mut node = TreeNode::new(nums[mid as usize]);

            node.left = helper(nums, start, mid - 1);
            node.right = helper(nums, mid+1, end);

            return Some(Rc::new(RefCell::new(node)));


        }

        helper(&nums, 0 , nums.len() as i32 - 1)

    }
}