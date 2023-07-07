use std::cell::RefCell;
use std::rc::Rc;

type SingleLink = Option<Rc<RefCell<Node>>>;

struct Node {
    value: String,
    next: SingleLink,
}

struct TransactionLog {
    head: SingleLink,
    tail: SingleLink,
    pub length: u64,
}

impl Node {
    pub fn new(value: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node { value, next: None }))
    }
}

impl TransactionLog {
    pub fn new_empty() -> TransactionLog {
        TransactionLog {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn append(&mut self, value: String) {
        match &self.tail {
            None => {
                let node = Node::new(value);
                self.tail = Some(node.clone());
                self.head = Some(node);
                self.length += 1;
            }
            Some(tail) => {
                let new_node = Node::new(value);
                tail.borrow_mut().next = Some(new_node.clone());
                self.tail = Some(new_node);
                self.length += 1;
            }
        }
    }
}
