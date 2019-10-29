use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

#[derive(Debug)]
pub struct RawNode<T> {
    data: T,
    pred: Weak<RefCell<RawNode<T>>>,
    suss: Weak<RefCell<RawNode<T>>>,
}

impl<T> RawNode<T> {
    pub fn new(data: T) -> Self {
        RawNode {
            data: data,
            pred: Weak::new(),
            suss: Weak::new()
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

    fn data(&self) -> &T {
        &self.data
    }
}

// #[derive(Debug)]
// pub struct SharedNode<T> (RefCell<RawNode<T>>);

// impl<T> From<RawNode<T>> for SharedNode<T> {
//     fn from(node: RawNode<T>) -> Self {
//         SharedNode(RefCell::new(node))
//     }
// }

#[derive(Debug)]
pub struct ListNode<T> (Rc<RefCell<RawNode<T>>>);

impl<T> From<RawNode<T>> for ListNode<T> {
    fn from(node: RawNode<T>) -> Self {
        ListNode(Rc::new(RefCell::new(node)))
    }
}

impl<T> ListNode<T> {
    pub fn new(data: T) -> Self {
        ListNode::from(RawNode::new(data))
    }

    pub fn set(&self, data: T) -> &Self {
        self.0.borrow_mut().data = data;
        self
    }

    pub fn get(&self) -> &T {
        let p = self.0.as_ptr();
        unsafe {
            p.as_ref().unwrap().data()
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