use std::{cell::RefCell, rc::Rc};

// 主函数 - 程序入口点
fn main() {
    // 创建一个简单的二叉树进行测试
    // 树结构：
    //     1
    //    / \
    //   2   3
    //  / \
    // 4   5
    let node4 = Rc::new(RefCell::new(TreeNode {
        val: 4,
        left: None,
        right: None,
    }));

    let node5 = Rc::new(RefCell::new(TreeNode {
        val: 5,
        left: None,
        right: None,
    }));

    let node2 = Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: Some(node4),
        right: Some(node5),
    }));

    let node3 = Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: None,
        right: None,
    }));

    let root = Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(node2),
        right: Some(node3),
    }));

    // 执行中序遍历
    let result = inorder_traversal(Some(root));

    // 打印结果
    println!("中序遍历结果: {:?}", result);
    // 预期输出: [4, 2, 5, 1, 3]
    // 解释：
    // 1. 先遍历左子树 (2 -> 4,5): 4 -> 2 -> 5
    // 2. 然后访问根节点: 1
    // 3. 最后遍历右子树: 3
}

// 定义二叉树节点结构
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,                             // 节点存储的值
    pub left: Option<Rc<RefCell<TreeNode>>>,  // 左子节点，使用Rc<RefCell>实现共享所有权和内部可变性
    pub right: Option<Rc<RefCell<TreeNode>>>, // 右子节点
}

pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut current = root;
    let mut ret = Vec::new();
    let mut stack = Vec::new();

    while current.is_some() || !stack.is_empty() {
        while let Some(node) = current {
            stack.push(node.clone());
            current = node.borrow().left.clone();
        }

        if let Some(node) = stack.pop() {
            ret.push(node.borrow().val);
            current = node.borrow().right.clone()
        }
    }
    ret
}

pub fn _inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    fn recursive(node: Option<Rc<RefCell<TreeNode>>>, ret: &mut Vec<i32>) {
        if let Some(node) = node {
            recursive(node.borrow().left.clone(), ret);
            ret.push(node.borrow().val);
            recursive(node.borrow().right.clone(), ret);
        }
    }
    let mut ret = Vec::new();
    recursive(root, &mut ret);
    ret
}
