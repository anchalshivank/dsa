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
    pub fn binary_tree_paths(root: Option<A>) -> Vec<String> {

        fn dfs(node: Option<A> , path: String, result: &mut Vec<String>){

            if let Some(node_rc) = node{

                let node_b = node_rc.borrow();
                // let n = path.len();

                let new_path = if path.len() != 0 {format!("{}->{}", path,node_b.val)}else{format!("{}", node_b.val)};
                
                if node_b.left.is_none() && node_b.right.is_none(){
                    result.push(new_path);
                    return;
                }

          
                
                dfs(node_b.left.clone(), new_path.clone(), result);
                dfs(node_b.right.clone(), new_path, result);
            }else{
                //we found the leaf node
                return;
            }

        }

        let mut result = Vec::new();
        let path = String::new();
        dfs(root, path, &mut result);
        result

        
    }
}