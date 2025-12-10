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
    // 如果根节点为空，返回 Nones
    root.as_ref()?;

    // 获取根节点的引用
    let root_rc = root.unwrap();

    // 如果根节点是 p 或 q 中的任意一个，返回根节点
    if Rc::ptr_eq(&root_rc, p.as_ref().unwrap()) || Rc::ptr_eq(&root_rc, q.as_ref().unwrap()) {
        return Some(root_rc);
    }

    // 递归查找左子树中的 LCA
    let left = lowest_common_ancestor(root_rc.borrow().left.clone(), p.clone(), q.clone());

    // 递归查找右子树中的 LCA
    let right = lowest_common_ancestor(root_rc.borrow().right.clone(), p.clone(), q.clone());

    // 如果左子树和右子树都找到了 LCA，则当前根节点是 LCA
    if left.is_some() && right.is_some() {
        return Some(root_rc);
    }

    // 如果只有一个子树找到了，返回那个结果，否则返回 None
    left.or(right)
}

fn main() {}
