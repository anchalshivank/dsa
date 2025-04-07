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
    pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {


        fn construct(nums: &Vec<i32> , start: usize, end: usize) -> Option<Rc<RefCell<TreeNode>>>{


            if start>=end{
                return None;
            }

            //first we need to find the max value!!
            //let's loop for now
            let mut max = i32::MIN;
            let mut mi = start;
            // let muit mi = 
            // println!("{}-{}", start, end);
            for (index, &num) in nums[start..end].iter().enumerate(){

                let i = index+ start;

                if num > max{
                    max = num;
                    mi = i;
                }

            }

            // println!("max {max}");


            //Now we know that is the max index


            let left_node = construct(nums, start, mi);
            let right_node = construct(nums, mi+1, end);

            let mut node = TreeNode::new(nums[mi]);
            node.left = left_node;
            node.right = right_node;

            Some(Rc::new(RefCell::new(node)))

        }


        construct(&nums, 0, nums.len())
        

    }
}