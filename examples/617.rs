use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    // 测试用例1: 两个简单的树
    let tree1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
    })));

    let tree2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    })));

    let merged = merge_trees(tree1, tree2);
    println!("测试用例1 - 合并结果: {:?}", merged);

    // 测试用例2: 一个树为空
    let tree3 = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    let tree4 = None;

    let merged2 = merge_trees(tree3, tree4);
    println!("测试用例2 - 一个树为空: {:?}", merged2);

    // 测试用例3: 两个树都为空
    let merged3 = merge_trees(None, None);
    println!("测试用例3 - 两个树都为空: {:?}", merged3);
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
