use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;
use std::default::Default;

use crate::node::Node;

#[derive(Debug)]
struct RawNode<T> {
    data: Option<Node<T>>,
    pred: Option<Rc<RefCell<RawNode<T>>>>,
    suss: Weak<RefCell<RawNode<T>>>,
}

impl<T> RawNode<T> {
    fn new(data: T) -> Self {
        RawNode {
            data: Some(Node::new(data)),
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

    pub fn as_ptr(&self) -> Option<*mut T> {
        match self.data.clone() {
            Some(data) => Some(data.as_ptr()),
            None => None
        }
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

impl<T> Default for ListNode<T> {
    fn default() -> Self {
        ListNode::from(RawNode::default())
    }
}

impl<T> From<RawNode<T>> for ListNode<T> {
    fn from(node: RawNode<T>) -> Self {
        ListNode(Rc::new(RefCell::new(node)))
    }
}

impl<T: std::fmt::Debug> ListNode<T> {
    pub(super) fn new(data: T) -> Self {
        Self::from(RawNode::new(data))
    }

    pub(super) fn insert_as_pred(&self, data: T) -> Self {
        let pred = Self::new(data);
        self.link_before(&pred);
        pred
    }

    pub(super) fn insert_as_suss(&self, data: T) -> Self {
        let suss = Self::new(data);
        self.link_after(&suss);
        suss
    }

    fn print_node(&self) -> String {
        match self.get() {
            None => String::from("None"),
            Some(data) => format!("{:?}", data)
        }
    }

    // pub(super) fn get_node(&self) -> Option<&Node<T>> {
    //     let ptr = self.0.as_ptr();
    //     unsafe {
    //         ptr.as_ref().unwrap().get()
    //     }
    // }

    pub(super) fn set_node(&self, node: Node<T>) -> &Self {
        self.0.borrow_mut().data = Some(node);
        self
    }

    pub(super) fn remove_node(&self) -> Option<Node<T>> {
        let mut p = self.0.borrow_mut();
        if let Some(node) = p.data.clone() {
            p.data = None;
            Some(node)
        } else {
            None
        }
    }

    pub fn strong_count(&self) -> usize {
        Rc::strong_count(&self.0)
    }

    pub(super) fn me(&self) -> Option<ListNode<T>> {
        Some(ListNode(self.0.clone()))
    }

    pub(super) fn pred(&self) -> Option<ListNode<T>> {
        self.0.borrow().pred()
    }

    pub(super) fn suss(&self) -> Option<ListNode<T>> {
        self.0.borrow().suss()
    }

    fn check_alone(&self, p: &Self) -> bool {
        Rc::ptr_eq(&self.0, &p.0) || p.pred().is_some() || p.suss().is_some()
    }

    pub(super) fn combine(&self, p: &Self) -> &Self {
        self.0.borrow_mut().suss = Rc::downgrade(&p.0);
        p.0.borrow_mut().pred = Some(Rc::clone(&self.0));
        self
    }

    pub(super) fn link_after(&self, p: &Self) -> &Self {
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

    pub(super) fn link_before(&self, p: &Self) -> &Self {
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

    pub(super) fn clean_after(&self) -> &Self {
        if let Some(q) = self.suss() {
            q.0.borrow_mut().pred = None;
        }
        self.0.borrow_mut().suss = Weak::new();
        self
    }

    pub(super) fn clean_before(&self) -> &Self {
        if let Some(p) = self.pred() {
            p.0.borrow_mut().suss = Weak::new();
        }
        self.0.borrow_mut().pred = None;
        self
    }

    pub fn get(&self) -> Option<&T> {
        let ptr = self.0.as_ptr();
        unsafe {
            let pptr = ptr.as_ref().unwrap().as_ptr().unwrap();
            pptr.as_ref()
        }
    }

    pub fn get_mut(&self) -> Option<&mut T> {
        let ptr = self.0.as_ptr();
        unsafe {
            let pptr = ptr.as_ref().unwrap().as_ptr().unwrap();
            pptr.as_mut()
        }
    }

    pub(super) fn as_ptr(&self) -> Option<*mut T> {
        let ptr = self.0.as_ptr();
        unsafe {
            ptr.as_ref().unwrap().as_ptr()
        }
    }

}

impl<T> Eq for ListNode<T> {}

impl<T> PartialEq for ListNode<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
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
            preds.push(format!("{}", rc.print_node()));
            q = rc.pred();
        }

        // print preds
        for pred in preds.iter().rev() {
            res = write!(f, "{} -> ", pred);
        }
        
        // print self
        res = write!(f, "(*){}", self.print_node());

        // print suss
        while let Some(rc) = p {
            res = write!(f, " -> {}", rc.print_node());
            p = rc.suss();
        }
        res
    }
}

impl<T: fmt::Display+fmt::Debug> fmt::Display for ListNode<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res = write!(f, "{}", self.print_node());
        res
    }
}