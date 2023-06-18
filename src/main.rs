struct Node {
    pub predicate: fn() -> bool,
    pub value: Option<i32>,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

fn left() -> bool { false }

impl Node {
    fn new(predicate: fn() -> bool) -> Node {
        Self { predicate, value: None, left: None, right: None }
    }

    fn new_terminal(value: i32) -> Node {
        Self { predicate: left, value: Some(value), left: None, right: None }
    }

    fn link(&mut self, node: Node, is_left: bool) {
        if is_left {
            self.left = Some(Box::new(node))
        } else {
            self.right = Some(Box::new(node))
        }
    }

    fn traverse(&mut self) -> i32 {
        let is_terminal = self.value.is_some();
        return if is_terminal {
            self.value.unwrap()
        } else if (self.predicate)() {
            self.right.as_mut()
                .map(|c| c.traverse())
                .unwrap()
        } else {
            self.left.as_mut()
                .map(|c| c.traverse())
                .unwrap()
        };
    }
}

fn main() {
    let mut node = Node::new(left);
    let mut left_child = Node::new(left);
    let mut right_child = Node::new(left);
    let leaf_1 = Node::new_terminal(10);
    let leaf_2 = Node::new_terminal(20);
    left_child.link(leaf_1, true);
    right_child.link(leaf_2, true);
    node.link(left_child, true);
    node.link(right_child, false);
    println!("{}", node.traverse());
}