pub struct List {
    head: Link,
}

enum Link {
    Terminating,
    Connecting(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    // TODO
    pub fn new() -> Self {
        List {
            head: Link::Terminating,
        }
    }

    pub fn push(&mut self, val: i32) {
        let node = Box::new(Node {
            elem: val,
            next: std::mem::replace(&mut self.head, Link::Terminating),
        });

        self.head = Link::Connecting(node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match std::mem::replace(&mut self.head, Link::Terminating) {
            Link::Terminating => None,
            Link::Connecting(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn initializes() {
        let _list = List::new();
    }

    #[test]
    fn push_new_elem() {
        let mut list = List::new();
        assert_eq!(list.pop(), None);
        list.push(11);
        list.push(12);
        list.push(13);
        assert_eq!(list.pop(), Some(13));
        assert_eq!(list.pop(), Some(12));
        assert_eq!(list.pop(), Some(11));
        assert_eq!(list.pop(), None);
    }
}
