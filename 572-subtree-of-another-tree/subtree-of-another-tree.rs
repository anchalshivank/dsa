use std::rc::Rc;
use std::cell::RefCell;

type A = Rc<RefCell<TreeNode>>;

impl Solution {
    pub fn is_subtree(root: Option<A>, sub_root: Option<A>) -> bool {
        // Compare if two trees are exactly the same
        fn is_same(tree1: Option<A>, tree2: Option<A>) -> bool {
            match (tree1, tree2) {
                (Some(a), Some(b)) => {
                    let (a, b) = (a.borrow(), b.borrow());
                    a.val == b.val
                        && is_same(a.left.clone(), b.left.clone())
                        && is_same(a.right.clone(), b.right.clone())
                }
                (None, None) => true,
                _ => false,
            }
        }

        // Traverse the main tree and check for subtree match
        fn traverse(node: Option<A>, sub: Option<A>) -> bool {
            match node {
                Some(n) => {
                    let n_borrow = n.borrow();
                    is_same(Some(n.clone()), sub.clone())
                        || traverse(n_borrow.left.clone(), sub.clone())
                        || traverse(n_borrow.right.clone(), sub)
                }
                None => false,
            }
        }

        traverse(root, sub_root)
    }
}
