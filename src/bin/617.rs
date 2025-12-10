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
    fn recursive(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match (root1, root2) {
            (None, None) => None,
            (Some(node1), None) => Some(node1),
            (None, Some(node2)) => Some(node2),
            (Some(node1), Some(node2)) => {
                let mut node1_ref = node1.borrow_mut();
                let mut node2_ref = node2.borrow_mut();

                let merged_node =
                    Rc::new(RefCell::new(TreeNode::new(node1_ref.val + node2_ref.val)));

                merged_node.borrow_mut().left =
                    recursive(node1_ref.left.take(), node2_ref.left.take());

                merged_node.borrow_mut().right =
                    recursive(node1_ref.right.take(), node2_ref.right.take());

                Some(merged_node)
            }
        }
    }

    recursive(root1, root2)
}
