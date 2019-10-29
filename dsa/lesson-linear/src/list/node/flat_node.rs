use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;
use std::collections::HashMap;

use crate::node::Node;

#[derive(Debug)]
pub struct RawNode<T> {
    data: Option<Node<T>>,
    pred: Weak<RefCell<RawNode<T>>>,
    suss: Weak<RefCell<RawNode<T>>>,
    next: Option<Rc<RefCell<RawNode<T>>>>
}

impl<T> RawNode<T> {
    pub fn new(data: T) -> Self {
        RawNode {
            data: Some(Node::new(data)),
            pred: Weak::new(),
            suss: Weak::new(),
            next: None,
        }
    }

    fn pred(&self) -> Option<ListNode<T>> {
        if let Some(node) = self.pred.upgrade() {
            Some(ListNode(node.clone()))
        } else {
            None
        }
    }

    fn suss(&self) -> Option<ListNode<T>> {
        if let Some(node) = self.suss.upgrade() {
            Some(ListNode(node.clone()))
        } else {
            None
        }
    }

    fn get(&self) -> Option<&Node<T>> {
        self.data.as_ref()
    }
}

#[derive(Debug)]
pub struct ListNode<T> (Rc<RefCell<RawNode<T>>>);

impl<T> From<RawNode<T>> for ListNode<T> {
    fn from(node: RawNode<T>) -> Self {
        ListNode(Rc::new(RefCell::new(node)))
    }
}

impl<T: std::fmt::Debug> ListNode<T> {
    pub fn new(data: T) -> Self {
        Self::from(RawNode::new(data))
    }

    pub fn insert_as_pred(&self, data: T) -> Self {
        let pred = Self::new(data);

        // 维护ownership链
        if let Some(next) = self.0.borrow_mut().next.clone() {
            pred.0.borrow_mut().next = Some(next);
        }
        self.0.borrow_mut().next = Some(pred.0.clone());

        pred.link(self);
        pred
    }

    pub fn insert_as_suss(&self, data: T) -> Self {
        let suss = Self::new(data);

        // 维护ownership链
        if let Some(next) = self.0.borrow_mut().next.clone() {
            suss.0.borrow_mut().next = Some(next);
        }
        self.0.borrow_mut().next = Some(suss.0.clone());

        self.link(&suss);
        suss
    }

    pub fn get_node(&self) -> Option<&Node<T>> {
        let ptr = self.0.as_ptr();
        unsafe {
            ptr.as_ref().unwrap().get()
        }
    }

    pub fn remove_node(&self) -> Option<Node<T>> {
        let mut p = self.0.borrow_mut();
        if let Some(node) = p.data.clone() {
            p.data = None;
            Some(node)
        } else {
            None
        }
    }

    pub fn pred(&self) -> Option<ListNode<T>> {
        self.0.borrow().pred()
    }

    pub fn suss(&self) -> Option<ListNode<T>> {
        self.0.borrow().suss()
    }

    pub fn link(&self, p: &Self) -> &Self {
        self.0.borrow_mut().suss = Rc::downgrade(&p.0);
        p.0.borrow_mut().pred = Rc::downgrade(&self.0);
        self
    }

}