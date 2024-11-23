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
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        Self::generate_trees_range(1,n)
    }

    pub fn generate_trees_range(start: i32, end: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>>{

        if start> end {
            return vec![None];
        }
        let mut res = vec![];

        for curr_val in start..=end{

            let left_trees  = Self::generate_trees_range(start, curr_val-1);
            let right_trees = Self::generate_trees_range(curr_val+1, end);

            let trees = Self::combine(curr_val, left_trees, right_trees);
            
            res.extend_from_slice(&trees);

        }

        res

    }


    pub fn combine(curr_val: i32, left_trees: Vec<Option<Rc<RefCell<TreeNode>>>>, right_trees :Vec<Option<Rc<RefCell<TreeNode>>>> ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {

    // let mut vec = vec![];

    // for left_tree in &left_trees{
    //     for right_tree in &right_trees{

    //         let curr = Rc::new(RefCell::new(TreeNode::new(curr_val)));
            
    //         curr.borrow_mut().left = left_tree.clone();
    //         curr.borrow_mut().right = right_tree.clone();
            
    //         vec.push(Some(curr));
            


    //     }
    // }


    let results = left_trees.iter().map(|left_tree| {
        right_trees.iter().map(|right_tree|{
            
            let curr = Rc::new(RefCell::new(TreeNode::new(curr_val)));
            curr.borrow_mut().right = right_tree.clone();
            curr.borrow_mut().left = left_tree.clone();
            Some(curr)
            
        }).collect::<Vec<_>>()
    });

    results.flatten().collect()




    // vec
}
}