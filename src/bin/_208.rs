use std::collections::HashMap;

#[derive(Default)]
struct Node {
    children: HashMap<char, Box<Node>>,
    is_end: bool,
}

struct Trie {
    root: Node,
}

impl Trie {
    fn new() -> Self {
        Self {
            root: Node::default(),
        }
    }

    fn insert(&self, word: String) {}

    fn search(&self, word: String) -> bool {}

    fn starts_with(&self, prefix: String) -> bool {}
}
