#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;
pub fn lowest_common_ancestor(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    let p = p.unwrap();
    let q = q.unwrap();

    fn recurse(
        node: Option<Rc<RefCell<TreeNode>>>,
        p: Rc<RefCell<TreeNode>>,
        q: Rc<RefCell<TreeNode>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = node {
            if Rc::ptr_eq(&node, &p) || Rc::ptr_eq(&node, &q) {
                return Some(node.clone());
            }
            let node_borrow = node.borrow();
            let left = recurse(node_borrow.left.clone(), p.clone(), q.clone());
            let right = recurse(node_borrow.right.clone(), p.clone(), q.clone());

            if left.is_some() && right.is_some() {
                Some(node.clone())
            } else {
                left.or(right)
            }
        } else {
            None
        }
    }

    recurse(root, p.clone(), q.clone())
}

fn main() {}
