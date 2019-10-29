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
    next: Option<Rc<RefCell<RawNode<T>>>>,
    owner: Weak<RefCell<RawNode<T>>>,
}

impl<T> RawNode<T> {
    pub fn new(data: T) -> Self {
        RawNode {
            data: Some(Node::new(data)),
            pred: Weak::new(),
            suss: Weak::new(),
            next: None,
            owner: Weak::new(),
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

    fn next(&self) -> Option<ListNode<T>> {
        if let Some(node) = self.next.clone() {
            Some(ListNode(node))
        } else {
            None
        }
    }

    fn owner(&self) -> Option<ListNode<T>> {
        if let Some(node) = self.owner.upgrade() {
            Some(ListNode(node.clone()))
        } else {
            None
        }
    }

    fn get(&self) -> Option<&Node<T>> {
        self.data.as_ref()
    }
}

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
        if let Some(next) = self.next() {
            pred.0.borrow_mut().next = Some(next.0.clone());
            next.0.borrow_mut().owner = Rc::downgrade(&pred.0);
        }
        self.0.borrow_mut().next = Some(pred.0.clone());
        pred.0.borrow_mut().owner = Rc::downgrade(&self.0);

        self.link_before(&pred);
        pred
    }

    pub fn insert_as_suss(&self, data: T) -> Self {
        let suss = Self::new(data);

        // 维护ownership链
        if let Some(next) = self.next() {
            suss.0.borrow_mut().next = Some(next.0.clone());
            next.0.borrow_mut().owner = Rc::downgrade(&suss.0);
        }
        self.0.borrow_mut().next = Some(suss.0.clone());
        suss.0.borrow_mut().owner = Rc::downgrade(&self.0);

        self.link_after(&suss);
        suss
    }

    fn print_node(&self) -> String {
        match self.get_node() {
            None => String::from("None"),
            Some(node) => format!("{:?}", node)
        }
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

    pub fn me(&self) -> Option<ListNode<T>> {
        Some(ListNode(self.0.clone()))
    }

    pub fn pred(&self) -> Option<ListNode<T>> {
        self.0.borrow().pred()
    }

    pub fn suss(&self) -> Option<ListNode<T>> {
        self.0.borrow().suss()
    }

    fn next(&self) -> Option<ListNode<T>> {
        self.0.borrow().next()
    }

    fn owner(&self) -> Option<ListNode<T>> {
        self.0.borrow().owner()
    }

    pub fn remove_ownership(&self) -> &Self {
        if let Some(p) = self.owner() {
            p.0.borrow_mut().next = self.0.borrow_mut().next.clone();
            if let Some(q) = self.next() {
                q.0.borrow_mut().owner = Rc::downgrade(&p.0);
            }
        }
        self.0.borrow_mut().owner = Weak::new();
        self.0.borrow_mut().next = None;

        self
    }

    pub fn insert_ownership(&self, p: &Self) -> &Self {
        if !Rc::ptr_eq(&self.0, &p.0) {
            p.0.borrow_mut().owner = Rc::downgrade(&self.0);
            p.0.borrow_mut().next = self.0.borrow_mut().next.clone();
            if let Some(q) = self.next() {
                q.0.borrow_mut().owner = Rc::downgrade(&p.0);
            }
            self.0.borrow_mut().next = Some(p.0.clone());
        }
        self
    }

    pub fn link_after(&self, p: &Self) -> &Self {
        p.0.borrow_mut().suss = self.0.borrow_mut().suss.clone();
        if let Some(q) = self.suss() {
            q.0.borrow_mut().pred = Rc::downgrade(&p.0);
        }
        self.0.borrow_mut().suss = Rc::downgrade(&p.0);
        p.0.borrow_mut().pred = Rc::downgrade(&self.0);
        self
    }

    pub fn link_before(&self, p: &Self) -> &Self {
        p.0.borrow_mut().pred = self.0.borrow_mut().pred.clone();
        if let Some(q) = self.pred() {
            q.0.borrow_mut().suss = Rc::downgrade(&p.0);
        }
        self.0.borrow_mut().pred = Rc::downgrade(&p.0);
        p.0.borrow_mut().suss = Rc::downgrade(&self.0);
        self
    }

    pub fn clear_after(&self) -> &Self {
        if let Some(q) = self.suss() {
            q.0.borrow_mut().pred = Weak::new();
        }
        self.0.borrow_mut().suss = Weak::new();
        self
    }

    pub fn clear_before(&self) -> &Self {
        if let Some(p) = self.pred() {
            p.0.borrow_mut().suss = Weak::new();
        }
        self.0.borrow_mut().pred = Weak::new();
        self
    }

}

use std::fmt;

impl<T: fmt::Debug> fmt::Debug for ListNode<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res = write!(f, "List: ");
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
        res = write!(f, " ");
        res = write!(f, "`Ownership: ");

        let mut mp = self.owner();
        let mut owners: Vec<String> = vec![];

        while let Some(p) = mp {
            owners.push(format!("{}", p.print_node()));
            mp = p.owner();
        }

        for owner in owners.iter().rev() {
            res = write!(f, "{} -> ", owner);
        }

        res = write!(f, "(*){}", self.print_node());

        let mut mp = self.next();

        while let Some(p) = mp {
            res = write!(f, " -> {}", p.print_node());
            mp = p.next();
        }

        res = write!(f, "`");
        res
    }
}