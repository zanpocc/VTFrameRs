use crate::memory::{npp::NPP, AllocationError};

#[repr(C)]
pub struct LinkedList<T> {
    head: Option<NPP<Node<T>>>,
    size: usize,
}

#[repr(C)]
pub struct Node<T> {
    pub val: T,
    pub next: Option<NPP<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            size: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
    pub fn len(&self) -> usize {
        self.size
    }

    pub fn push(&mut self, data: T) -> Result<&mut Self, AllocationError> {
        let node = NPP::new(Node {
            val: data,
            next: self.head.take(), // 头插法，指向之前的第一个元素
        })?;

        self.head = Some(node);
        self.size += 1;
        Ok(self)
    }

    pub fn into_iter(&mut self) -> IterMut<T> {
        IterMut {
            current: self.head.as_deref_mut(),
        }
    }
}

pub struct IterMut<'a, T> {
    current: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().map(|node| {
            self.current = node.next.as_deref_mut();
            &mut node.val
        })
    }
}
