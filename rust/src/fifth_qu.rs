use std::ptr;

pub struct List<T> {
    head: Link<T>,
    tail: *mut Node<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: ptr::null_mut(),
        }
    }

    pub fn push(&mut self, elem: T) {
        let mut new_tail = Box::new(Node { elem, next: None });
        let raw_tail: *mut _ = &mut *new_tail;

        if !self.tail.is_null() {
            unsafe {
                (*self.tail).next = Some(new_tail);
            }
        } else {
            self.head = Some(new_tail);
        }

        self.tail = raw_tail;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            let head = *head;
            self.head = head.next;

            if self.head.is_none() {
                self.tail = ptr::null_mut();
            }

            head.elem
        })
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
        assert_eq!(list.pop(), Some(11));
        assert_eq!(list.pop(), Some(12));
        assert_eq!(list.pop(), Some(13));
        assert_eq!(list.pop(), None);
    }
}
