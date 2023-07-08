use std::cell::RefCell;
use std::rc::Rc;

type SingleLink = Option<Rc<RefCell<Node>>>;

#[derive(Debug, PartialEq)]
struct Node {
    value: String,
    next: SingleLink,
}

#[derive(Debug)]
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
        let node = Node::new(value);
        match self.tail.take() {
            None => {
                self.head = Some(node.clone());
            }
            Some(tail) => {
                tail.borrow_mut().next = Some(node.clone());
            }
        }
        self.tail = Some(node);
        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<String> {
        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
                self.head = Some(next);
            } else {
                self.tail.take(); // why use take? I guess just to clean it up? probably equivalent to just setting it to None?
            }
            self.length -= 1;
            Rc::try_unwrap(head)
                .ok()
                .expect("It should just work")
                .into_inner() // Basically "unwrapping" the RefCell
                .value
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_appending() {
        let mut tl = TransactionLog::new_empty();
        assert!(tl.head.is_none());
        assert!(tl.tail.is_none());
        assert_eq!(tl.length, 0);
        tl.append(String::from("Testing1"));
        assert_eq!(tl.length, 1);
        assert_eq!(tl.head, Some(Node::new("Testing1".to_string()))); // node without a next
        assert_eq!(tl.tail, Some(Node::new("Testing1".to_string())));
        tl.append(String::from("Testing2"));
        assert_eq!(tl.length, 2);
        assert!(tl.head.clone().unwrap().borrow().next.is_some()); // head has a next now
        assert_eq!(
            tl.head.clone().unwrap().borrow().next,
            Some(Node::new(String::from("Testing2"))) // does not have a next
        );
        assert_eq!(tl.tail, Some(Node::new(String::from("Testing2"))));
        tl.append(String::from("Testing3"));
        assert_eq!(tl.length, 3);
        assert_eq!(
            tl.head
                .clone()
                .unwrap()
                .borrow()
                .next
                .clone()
                .unwrap()
                .borrow()
                .next,
            Some(Node::new(String::from("Testing3"))) // head is unchanged, but the chain groweth
        );
        assert_eq!(tl.tail, Some(Node::new(String::from("Testing3"))));
    }

    #[test]
    fn test_popping() {
        let mut tl = TransactionLog::new_empty();
        tl.append(String::from("Testing1"));
        tl.append(String::from("Testing2"));
        tl.append(String::from("Testing3"));

        assert_eq!(tl.pop(), Some("Testing1".to_string()));
        assert_eq!(tl.length, 2);
        assert!(tl.head.clone().unwrap().borrow().next.is_some());
        assert_eq!(
            tl.head.clone().unwrap().borrow().next,
            Some(Node::new(String::from("Testing3"))) // Testing2 is the head now, and Testing3 is its next
        );
        assert_eq!(tl.tail, Some(Node::new(String::from("Testing3"))));
        assert_eq!(tl.pop(), Some(String::from("Testing2")));
        assert_eq!(tl.length, 1);
        assert_eq!(tl.pop(), Some(String::from("Testing3")));
        assert_eq!(tl.length, 0);
        assert_eq!(tl.head, None);
        assert!(tl.tail.is_none());
    }
}
