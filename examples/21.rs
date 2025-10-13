fn main() {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut dummy = ListNode::new(0);
    let mut current = &mut dummy;
    let mut l1 = list1;
    let mut l2 = list2;

    while l1.is_some() && l2.is_some() {
        let l1_val = l1.as_ref().unwrap().val;
        let l2_val = l2.as_ref().unwrap().val;

        if l1_val <= l2_val {
            let next = l1.as_mut().unwrap().next.take();
            current.next = l1;
            l1 = next;
        } else {
            let next = l2.as_mut().unwrap().next.take();
            current.next = l2;
            l2 = next;
        }
        current = current.next.as_mut().unwrap();
    }

    if l1.is_some() {
        current.next = l1;
    } else {
        current.next = l2;
    }

    dummy.next
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_list(values: &[i32]) -> Option<Box<ListNode>> {
        let mut head = None;
        for &val in values.iter().rev() {
            let mut node = ListNode::new(val);
            node.next = head;
            head = Some(Box::new(node));
        }
        head
    }

    fn list_to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut result = Vec::new();
        while let Some(node) = head {
            result.push(node.val);
            head = node.next;
        }
        result
    }

    #[test]
    fn test_merge_two_lists_basic() {
        let list1 = create_list(&[1, 3, 5]);
        let list2 = create_list(&[2, 4, 6]);
        let merged = merge_two_lists(list1, list2);
        assert_eq!(list_to_vec(merged), vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_merge_empty_lists() {
        let list1 = create_list(&[]);
        let list2 = create_list(&[]);
        let merged = merge_two_lists(list1, list2);
        assert_eq!(list_to_vec(merged), vec![]);
    }

    #[test]
    fn test_merge_one_empty_list() {
        let list1 = create_list(&[1, 2, 3]);
        let list2 = create_list(&[]);
        let merged = merge_two_lists(list1, list2);
        assert_eq!(list_to_vec(merged), vec![1, 2, 3]);
    }

    #[test]
    fn test_merge_duplicate_values() {
        let list1 = create_list(&[1, 1, 3]);
        let list2 = create_list(&[1, 2, 2]);
        let merged = merge_two_lists(list1, list2);
        assert_eq!(list_to_vec(merged), vec![1, 1, 1, 2, 2, 3]);
    }

    #[test]
    fn test_merge_single_element_lists() {
        let list1 = create_list(&[5]);
        let list2 = create_list(&[3]);
        let merged = merge_two_lists(list1, list2);
        assert_eq!(list_to_vec(merged), vec![3, 5]);
    }
}
