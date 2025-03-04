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
use std::collections::HashMap;
use std::cell::RefCell;

type A = Rc<RefCell<TreeNode>>;

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<A> {
        let mut map = HashMap::new();
        
        // Store index of each inorder value
        for (index, &val) in inorder.iter().enumerate() {
            map.insert(val, index);
        }

        fn helper(
            preorder: &Vec<i32>, 
            map: &HashMap<i32, usize>, 
            index: &mut usize, 
            left: i32, 
            right: i32
        ) -> Option<A> {
            // Base case: If the range is invalid, return None
            if left > right {
                return None;
            }

            // Get the current root value from preorder
            let val = preorder[*index];
            *index += 1;
            println!("index {}", *index);

            // Find the position of the root in the inorder array
            let mid = *map.get(&val).unwrap() as i32;

            // Construct the current node
            let mut node = TreeNode::new(val);

            // Recursively build the left subtree (only if valid range exists)
            node.left = helper(preorder, map, index, left, mid - 1);

            // Recursively build the right subtree (only if valid range exists)
            node.right = helper(preorder, map, index, mid + 1, right);

            Some(Rc::new(RefCell::new(node)))
        }

        let mut index = 0;
        helper(&preorder, &map, &mut index, 0, inorder.len() as i32 - 1)
    }
}