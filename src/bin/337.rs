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
pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(node: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        if let Some(n) = node {
            let n = n.borrow();
            let left = dfs(n.left.clone());
            let right = dfs(n.right.clone());
            let rob = n.val + left.1 + right.1;
            let not_rob = left.0.max(left.1) + right.0.max(right.1);
            (rob, not_rob)
        } else {
            (0, 0)
        }
    }
    let (rob, not_rob) = dfs(root);
    rob.max(not_rob)
}
pub fn _rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    // DFS
    fn recurse(node: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        if let Some(node) = node {
            let node_borrow = node.borrow();

            let left = recurse(node_borrow.left.clone());
            let right = recurse(node_borrow.right.clone());

            let rob = node_borrow.val + left.1 + right.1;
            let unrob = left.0.max(left.1) + right.0.max(right.1);

            (rob, unrob)
        } else {
            (0, 0)
        }
    }

    let (rob, unrob) = recurse(root);
    rob.max(unrob)
}

fn main() {}
