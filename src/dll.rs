use std::cell::RefCell;
use std::rc::Rc;
use std::fmt;

type NodePtr<T> = Rc<RefCell<DoublyLinkedListNode<T>>>;

struct DoublyLinkedListNode<T> {
    inner: T,
    prev: Option<NodePtr<T>>,
    next: Option<NodePtr<T>>,
}

pub struct DoublyLinkedList<T> {
    head: Option<NodePtr<T>>,
    tail: Option<NodePtr<T>>,
}

impl<T: Clone> DoublyLinkedList<T> {
    pub fn new() -> Self {
        DoublyLinkedList {
            head: None,
            tail: None,
        }
    }

    pub fn len(&self) -> usize {
        if self.head.is_some() {
            let mut cur_len = 0;
            let mut cur_node = self.head.clone();
            while let Some(node) = cur_node {
                cur_len += 1;
                cur_node = node.borrow().next.clone();
            }
            cur_len
        }
        else {
             0
        }
    }

    pub fn push(&mut self, item: T) {
        let new = match &self.tail {
            Some(node) => {
                let last = Rc::new(RefCell::new(DoublyLinkedListNode {
                    inner: item,
                    prev: Some(node.clone()),
                    next: None,
                }));
                node.borrow_mut().next = Some(last.clone());
                last
            },
            None => {
                assert!(self.head.is_none());
                let first = Rc::new(RefCell::new(DoublyLinkedListNode {
                    inner: item,
                    prev: None,
                    next: None,
                }));
                self.head = Some(first.clone());
                first
            }
        };
        self.tail = Some(new);
    }

    pub fn get(&self, idx: usize) -> T {
        let mut cur_idx = 0;
        let mut cur_node = self.head.clone();
        loop {
            match cur_node {
                Some(node) => {
                    if cur_idx == idx {
                        return node.borrow().inner.clone();
                    }
                    cur_idx += 1;
                    cur_node = node.borrow().next.clone();
                },
                None => panic!("Index {} out-of-bounds for DoublyLinkedList", cur_idx),
            }
        }
    }

    pub fn remove(&mut self, idx: usize) -> T {
        let mut cur_idx = 0;
        let mut cur_node = self.head.clone();
        loop {
            match cur_node {
                Some(node) => {
                    if cur_idx == idx {
                        assert!(!self.head.is_none() && !self.tail.is_none());
                        let head = self.head.as_ref().unwrap();
                        let tail = self.tail.as_ref().unwrap();
                        match node {
                            _ if Rc::ptr_eq(&node, head) => {
                                let node = node.borrow();
                                assert!(node.prev.is_none());

                                let next = node.next.clone();
                                if let Some(nxt_node) = &next {
                                    nxt_node.borrow_mut().prev = None;
                                }
                                else {
                                    self.tail = None;
                                }
                                self.head = next;
                            },
                            _ if Rc::ptr_eq(&node, tail) => {
                                let node = node.borrow();
                                assert!(node.next.is_none());

                                let prev = node.prev.clone();
                                if let Some(prv_node) = &prev {
                                    prv_node.borrow_mut().next = None;
                                }
                                else {
                                    self.head = None;
                                }
                                self.tail = prev;
                            },
                            _ => {
                                let node = node.borrow();
                                assert!(!node.next.is_none() && !node.prev.is_none());

                                let next = node.next.clone();
                                let prev = node.prev.clone();

                                if let Some(nxt_node) = &next {
                                    nxt_node.borrow_mut().prev = prev.clone();
                                }
                                if let Some(prv_node) = &prev {
                                    prv_node.borrow_mut().next = next;
                                }
                            }
                        }

                        return match Rc::try_unwrap(node) {
                            Ok(node) => node.into_inner().inner,
                            Err(_) => panic!("References remain to removed node"),
                        };
                    }
                    cur_idx += 1;
                    cur_node = node.borrow().next.clone();
                },
                None => panic!("Index {} out-of-bounds for DoublyLinkedList", cur_idx),
            }
        }
    }
}

impl<T: fmt::Debug>  fmt::Debug for DoublyLinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::from("[");

        let mut cur_node = self.head.clone();

        while let Some(node) = cur_node {
            buf.push_str(&format!("{:?}", &node.borrow().inner));
            cur_node = node.borrow().next.clone();
            if cur_node.is_some() {
                buf.push_str(", ");
            }
        }

        buf.push_str("]");

        write!(f, "{}", buf)
    }
}
