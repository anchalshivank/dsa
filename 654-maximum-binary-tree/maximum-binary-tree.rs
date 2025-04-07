use std::rc::Rc;
use std::cell::RefCell;

// Assuming TreeNode is already defined
impl Solution {
    pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();

        for num in nums {
            let curr = Rc::new(RefCell::new(TreeNode::new(num)));

            // Step 1: Pop all smaller nodes, they become left child
            while let Some(last) = stack.last() {
                if last.borrow().val < num {
                    let popped = stack.pop().unwrap();
                    curr.borrow_mut().left = Some(popped);
                } else {
                    break;
                }
            }

            // Step 2: Top of stack becomes parent, curr is right child
            if let Some(top) = stack.last() {
                top.borrow_mut().right = Some(Rc::clone(&curr));
            }

            // Step 3: Push current node to stack
            stack.push(curr);
        }

        // First element in the stack is root
        stack.first().cloned()
    }
}
