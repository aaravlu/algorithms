use std::cell::RefCell;
use std::rc::Rc;

fn main() {}

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

pub fn merge_trees(
    root1: Option<Rc<RefCell<TreeNode>>>,
    root2: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    fn recurse(
        node1: Option<Rc<RefCell<TreeNode>>>,
        node2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match (node1, node2) {
            (Some(node), None) | (None, Some(node)) => Some(node),
            (Some(node1), Some(node2)) => {
                let node1_borrow = node1.borrow();
                let node2_borrow = node2.borrow();

                let node1_val = node1_borrow.val;
                let node2_val = node2_borrow.val;

                let left = recurse(node1_borrow.left.clone(), node2_borrow.left.clone());
                let right = recurse(node1_borrow.right.clone(), node2_borrow.right.clone());

                Some(Rc::new(RefCell::new(TreeNode {
                    val: node1_val + node2_val,
                    left,
                    right,
                })))
            }
            (None, None) => None,
        }
    }

    recurse(root1, root2)
}
