pub struct List {
    head: Link,
}

enum Link {
    Terminating,
    Connecting(Box<List>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    // TODO
    pub fn new() -> Self {
        List { head: Link::Terminating }
    }

    pub fn push(&mut self, val: i32) {
    }

    pub fn get(&self, index: usize) {
    }
}

mod testing {
    use super::*;

    #[test]
    fn initializes() {
        let list = List::new();
    }

    fn push_new_elem() {
        let mut list = List::new();
        list.push(10);
        assert_eq!(list.get(0), 10);
    }
}
