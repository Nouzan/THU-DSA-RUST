use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;
use std::default::Default;

use crate::node::Node;

#[derive(Debug)]
struct RawNode<T> {
    data: Option<T>,
    pred: Option<Rc<RefCell<RawNode<T>>>>,
    suss: Weak<RefCell<RawNode<T>>>,
}

impl<T> RawNode<T> {
    fn new(data: T) -> Self {
        RawNode {
            data: Some(data),
            pred: None,
            suss: Weak::new(),
        }
    }

    fn pred(&self) -> Option<ListNode<T>> {
        if let Some(node) = self.pred.clone() {
            Some(ListNode(node))
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

    fn set(&mut self, data: T) -> &mut Self {
        self.data = Some(data);
        self
    }

    fn take(&mut self) -> Option<T> {
        self.data.take()
    }

}

impl<T> Default for RawNode<T> {
    fn default() -> Self {
        RawNode {
            data: None,
            pred: None,
            suss: Weak::new(),
        }
    }
}

pub struct ListNode<T> (Rc<RefCell<RawNode<T>>>);

impl<T> Node<T> for ListNode<T> {
    fn new(data: T) -> Self {
        Self::from(RawNode::new(data))
    }

    fn get(&self) -> Option<&T> {
        let ptr = self.0.as_ptr();
        unsafe {
            (*ptr).data.as_ref()
        }
    }

    fn get_mut(&mut self) -> Option<&mut T> {
        let ptr = self.0.as_ptr();
        unsafe {
            (*ptr).data.as_mut()
        }
    }

    fn set(&mut self, data: T) -> &mut Self {
        self.0.borrow_mut().set(data);
        self
    }

    fn take(&mut self) -> Option<T> {
        self.0.borrow_mut().take()
    }
}

impl<T> Default for ListNode<T> {
    fn default() -> Self {
        ListNode::from(RawNode::default())
    }
}

impl<T> Eq for ListNode<T> {}

impl<T> PartialEq for ListNode<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl<T> Clone for ListNode<T> {
    fn clone(&self) -> Self {
        ListNode(Rc::clone(&self.0))
    }
}

impl<T> From<RawNode<T>> for ListNode<T> {
    fn from(node: RawNode<T>) -> Self {
        ListNode(Rc::new(RefCell::new(node)))
    }
}

impl<T> ListNode<T> {
    pub fn insert_as_pred(&mut self, data: T) -> Self {
        let mut pred = Self::new(data);
        self.link_before(&mut pred);
        pred
    }

    pub fn insert_as_suss(&mut self, data: T) -> Self {
        let mut suss = Self::new(data);
        self.link_after(&mut suss);
        suss
    }

    fn strong_count(&self) -> usize {
        Rc::strong_count(&self.0)
    }

    fn me(&self) -> Option<ListNode<T>> {
        Some(ListNode(self.0.clone()))
    }

    pub fn pred(&self) -> Option<ListNode<T>> {
        self.0.borrow().pred()
    }

    pub fn suss(&self) -> Option<ListNode<T>> {
        self.0.borrow().suss()
    }

    fn check_alone(&self, p: &Self) -> bool {
        Rc::ptr_eq(&self.0, &p.0) || p.pred().is_some() || p.suss().is_some()
    }

    pub fn combine(&mut self, p: &mut Self) -> &mut Self {
        self.0.borrow_mut().suss = Rc::downgrade(&p.0);
        p.0.borrow_mut().pred = Some(Rc::clone(&self.0));
        self
    }

    pub fn link_after(&mut self, p: &mut Self) -> &mut Self {
        if !self.check_alone(p) {
            p.0.borrow_mut().suss = self.0.borrow_mut().suss.clone();
            if let Some(q) = self.suss() {
                q.0.borrow_mut().pred = Some(Rc::clone(&p.0));
            }
            self.0.borrow_mut().suss = Rc::downgrade(&p.0);
            p.0.borrow_mut().pred = Some(Rc::clone(&self.0));
        }
        self
    }

    pub fn link_before(&mut self, p: &mut Self) -> &mut Self {
        if !self.check_alone(p) {
            p.0.borrow_mut().pred = self.0.borrow_mut().pred.clone();
            if let Some(q) = self.pred() {
                q.0.borrow_mut().suss = Rc::downgrade(&p.0);
            }
            self.0.borrow_mut().pred = Some(Rc::clone(&p.0));
            p.0.borrow_mut().suss = Rc::downgrade(&self.0);
        }
        self
    }

    pub fn clean_after(&mut self) -> &mut Self {
        if let Some(q) = self.suss() {
            q.0.borrow_mut().pred = None;
        }
        self.0.borrow_mut().suss = Weak::new();
        self
    }

    pub fn clean_before(&mut self) -> &mut Self {
        if let Some(p) = self.pred() {
            p.0.borrow_mut().suss = Weak::new();
        }
        self.0.borrow_mut().pred = None;
        self
    }

}

use std::fmt;

impl<T: fmt::Debug> fmt::Debug for ListNode<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res = write!(f, "");
        let mut p = self.suss();
        let mut q = self.pred();
        let mut preds: Vec<String> = vec![];

        // collect preds
        while let Some(rc) = q {
            preds.push(format!("{:?}", rc.get()));
            q = rc.pred();
        }

        // print preds
        for pred in preds.iter().rev() {
            res = write!(f, "{} -> ", pred);
        }
        
        // print self
        res = write!(f, "(*){:?}", self.get());

        // print suss
        while let Some(rc) = p {
            res = write!(f, " -> {:?}", rc.get());
            p = rc.suss();
        }
        res
    }
}

impl<T: fmt::Display+fmt::Debug> fmt::Display for ListNode<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res = write!(f, "{:?}", self.get());
        res
    }
}