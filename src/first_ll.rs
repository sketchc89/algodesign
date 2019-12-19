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
}
