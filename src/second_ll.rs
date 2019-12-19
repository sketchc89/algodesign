pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}
impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }
}

impl<T> Drop for List<T> {
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
        let _list = List::<i32>::new();
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

    #[test]
    fn peek_at_head() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        list.push(10);
        assert_eq!(list.peek(), Some(&10));
    }

    #[test]
    fn peek_mut_at_head() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        list.push(10);
        assert_eq!(list.peek_mut(), Some(&mut 10));
    }
}
