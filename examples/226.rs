use std::{cell::RefCell, rc::Rc};

fn main() {
    // 测试反转二叉树
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 4,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 7,
            left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
        }))),
    })));

    println!("原始树:");
    print_tree(&root, 0);

    let inverted = invert_tree(root);

    println!("\n反转后的树:");
    print_tree(&inverted, 0);
}

fn print_tree(node: &Option<Rc<RefCell<TreeNode>>>, depth: usize) {
    if let Some(n) = node {
        let node_ref = n.borrow();
        println!("{}{}", "  ".repeat(depth), node_ref.val);
        print_tree(&node_ref.left, depth + 1);
        print_tree(&node_ref.right, depth + 1);
    }
}

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

pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(node) = root {
        let mut node_ref = node.borrow_mut();

        // 递归反转左右子树
        let left = invert_tree(node_ref.left.take());
        let right = invert_tree(node_ref.right.take());

        // 交换左右子树
        node_ref.left = right;
        node_ref.right = left;

        drop(node_ref); // 显式释放借用

        Some(node)
    } else {
        None
    }
}
