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
use std::collections::HashSet;
use std::rc::Rc;
pub fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut set = HashSet::new();

    fn recursive(node: Option<Rc<RefCell<TreeNode>>>, set: &mut HashSet<i32>) {
        if let Some(node) = node {
            set.insert(node.borrow().val);
            recursive(node.borrow().left.clone(), set);
            recursive(node.borrow().right.clone(), set);
        }
    }

    recursive(root, &mut set);

    set.len() == 1
}

fn main() {}
