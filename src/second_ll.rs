pub struct List {
    head: Link,
}

type Link = Option<Box<Node>>;

struct Node {
    elem: i32,
    next: Link,
}
impl List {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut link = self.head.take();
        while let Some(mut node) = link {
            link = node.next.take();
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
