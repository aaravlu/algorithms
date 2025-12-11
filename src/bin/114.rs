use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
    fn recurse(node: &mut Option<Rc<RefCell<TreeNode>>>) {
        if let Some(node) = node {
            let mut node_borrow = node.borrow_mut();
            recurse(&mut node_borrow.left);
            recurse(&mut node_borrow.right);

            if let Some(left) = node_borrow.left.take() {
                let mut current_right = left.clone();
                while let Some(next_right) = current_right.clone().borrow().right.clone() {
                    current_right = next_right
                }

                current_right.borrow_mut().right = node_borrow.right.clone();
                node_borrow.right = Some(left.clone())
            }
        }
    }
    recurse(root)
}

fn main() {}

#[cfg(test)]
mod tests {}
