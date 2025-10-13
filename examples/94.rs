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

// 二叉树中序遍历函数
// 中序遍历顺序：左子树 -> 根节点 -> 右子树
pub fn _inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    // 创建一个空向量来存储遍历结果
    let mut result = Vec::new();

    // 创建一个栈来辅助遍历（模拟递归的调用栈）
    let mut stack = Vec::new();

    // current指向当前正在处理的节点，初始化为根节点
    let mut current = root;

    // 主循环：当还有节点需要处理或栈不为空时继续
    // current.is_some(): 还有当前节点需要处理
    // !stack.is_empty(): 栈中还有待处理的节点
    while current.is_some() || !stack.is_empty() {
        // 内层循环：一直向左走，把所有左子节点压入栈中
        // 这模拟了递归遍历左子树的过程
        while let Some(node) = current {
            // 将当前节点压入栈中（保存起来，稍后处理）
            stack.push(node.clone());

            // 移动到左子节点，继续向左深入
            current = node.borrow().left.clone();
        }

        // 当不能再向左走时，从栈中弹出节点进行处理
        if let Some(node) = stack.pop() {
            // 访问当前节点（中序遍历的核心：在访问完左子树后访问根节点）
            // 将当前节点的值添加到结果列表中
            result.push(node.borrow().val);

            // 处理完当前节点后，转向右子树
            // 将current设置为右子节点，下一轮循环会处理右子树
            current = node.borrow().right.clone();
        }
    }

    // 返回中序遍历的结果
    result
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

// 算法思路解析：
// 1. 中序遍历的顺序是：左子树 -> 根节点 -> 右子树
// 2. 使用栈来模拟递归调用的过程
// 3. 核心思想：
//    - 先一直向左走，把所有左子节点压入栈中
//    - 当不能再向左时，弹出栈顶节点（这是最左边的节点）
//    - 访问该节点（添加到结果中）
//    - 然后转向该节点的右子树，重复上述过程
//
// 示例（二叉树：1 -> 2,3; 2 -> 4,5）：
// 遍历顺序：4 -> 2 -> 5 -> 1 -> 3
//
// 时间复杂度：O(n)，每个节点被访问一次
// 空间复杂度：O(h)，h是树的高度，最坏情况下是O(n)
