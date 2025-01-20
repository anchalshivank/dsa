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
    pub fn recover_tree(root: &mut A) {
        
        let mut first : A= None;
        let mut second : A= None;
        let mut prev : A= None;

        let mut current : A = root.clone();
        while let Some(ref curr_rc) = current.clone(){

            let curr_b = curr_rc.borrow();
            if curr_b.left.is_some(){

                //find the inorder pred

                let mut pred = curr_b.left.clone();

                while let Some(ref pred_rc) = pred.clone(){
                    let mut pred_b = pred_rc.borrow_mut();
                    if pred_b.right.is_some() {

                        if pred_b.right == current{

                            //delink
                            pred_b.right = None;
                            //println the curr thing
                            if let Some(ref prev_rc) = prev.clone(){
                                // println!("{:?}", prev_rc);
                                drop(pred_b);
                                let prev_val = prev_rc.borrow().val;
        
                                if prev_val > curr_b.val{
                                    if first.is_none(){
                                        first = Some(prev_rc.clone());

                                        second = Some(curr_rc.clone());

                                    }else {

                                        second = Some(curr_rc.clone());

                                    }
                                }


                            }
                            prev = Some(curr_rc.clone());
                            current = curr_b.right.clone();
                            break;
                        }else{

                            //else keep finding the inorder
                            pred = pred_b.right.clone();

                        }

                    }else{
                        //link to the current node
                        pred_b.right = current;
                        current = curr_b.left.clone();
                        break;
                    }

                }


            }else{
                //print here
                if let Some(ref prev_rc) = prev.clone(){

                                // println!("{:?}", prev_rc);
                    let prev_val = prev_rc.borrow().val;

                    if prev_val > curr_b.val{
                              if first.is_none(){
                                        first = Some(prev_rc.clone());

                                        second = Some(curr_rc.clone());

                                    }else {

                                        second = Some(curr_rc.clone());

                                    }
                    }


                }
                prev = Some(curr_rc.clone());
                current = curr_b.right.clone();

            }

        }

        if let (Some(r#fn), Some(sn)) = (first, second){
            let first_val = r#fn.borrow().val;
            let second_val = sn.borrow().val;

            r#fn.borrow_mut().val = second_val;
            sn.borrow_mut().val = first_val;
        }
    }
}