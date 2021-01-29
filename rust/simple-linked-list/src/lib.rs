use std::boxed::Box;
use std::iter::FromIterator;

#[derive(Debug)]
pub struct SimpleLinkedList<T: Clone> {
    head: Option<Box<Node<T>>>,
}

#[derive(Debug)]
struct Node<T>
where
    T: Clone,
{
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        let mut cur = self.head.as_ref();
        let mut count = 0;
        while let Some(node) = cur {
            count += 1;
            cur = node.next.as_ref();
        }
        count
    }

    pub fn push(&mut self, data: T) {
        let new_node = Box::new(Node {
            data,
            next: std::mem::replace(&mut self.head, None),
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        let old_head = std::mem::replace(&mut self.head, None);
        match old_head {
            None => return None,
            Some(mut node) => {
                std::mem::swap(&mut node.next, &mut self.head);
                Some(node.data)
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        match self.head {
            None => None,
            Some(ref node) => Some(&node.data),
        }
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut list = Self::new();
        let mut cur = self.head.as_ref();
        while let Some(node) = cur {
            list.push(node.data.clone());
            cur = node.next.as_ref();
        }

        list
    }
}

impl<T: Clone> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = Self::new();
        for v in iter {
            list.push(v);
        }
        list
    }
}

impl<T: Clone> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut vec = Vec::with_capacity(self.len());
        let mut cur = self.head.as_ref();
        while let Some(node) = cur {
            vec.push(node.data.clone());
            cur = node.next.as_ref();
        }
        vec.reverse();
        vec
    }
}
