use std::cell::RefCell;
use std::cell::Ref;
use std::rc::Rc;
use std::rc::Weak;
use std::fmt;
use std::ops::{Add, Deref, DerefMut};

pub mod flat_node;

pub struct ListNode<T> {
    data: T,
    pr: Weak<RefCell<ListNode<T>>>,
    su: Weak<RefCell<ListNode<T>>>,
    rc: Weak<RefCell<ListNode<T>>>
}

pub type ListNodePosi<T> = Rc<RefCell<ListNode<T>>>;

impl<T: fmt::Debug> ListNode<T> {
    fn new(e: T) -> Self {
        ListNode {
            data: e,
            pr: Weak::new(),
            su: Weak::new(),
            rc: Weak::new()
        }
    }

    fn to_posi(self) -> ListNodePosi<T> {
        let rc = Rc::new(RefCell::new(
            self
        ));
        rc.borrow_mut().rc = Rc::downgrade(&rc.clone());
        rc
    }

    fn get_posi(&self) -> Option<ListNodePosi<T>> {
        self.rc.upgrade()
    }

    pub fn move_out_data(self) -> T {
        self.data
    }

    pub fn into_inner(p: ListNodePosi<T>) -> T {
        Rc::try_unwrap(p).unwrap().into_inner().move_out_data()
    }

    pub fn new_as_posi(e: T) -> ListNodePosi<T> {
        let p = Self::new(e);
        p.to_posi()
    }

    pub fn insert_as_pred(&mut self, e: T) -> ListNodePosi<T> {
        let p = Self::new_as_posi(e);
        self.pr = Rc::downgrade(&p);
        if let Some(rc) = self.get_posi() {
            p.borrow_mut().su = Rc::downgrade(&rc);
        }
        p
    }

    pub fn insert_as_suss(&mut self, e: T) -> ListNodePosi<T> {
        let p = Self::new_as_posi(e);
        self.su = Rc::downgrade(&p);
        if let Some(rc) = self.get_posi() {
            p.borrow_mut().pr = Rc::downgrade(&rc);
        }
        p
    }

    pub fn combine(&mut self, o: &ListNodePosi<T>) -> &mut Self {
        self.su = Rc::downgrade(o);
        if let Some(rc) = self.get_posi() {
            o.borrow_mut().pr = Rc::downgrade(&rc);
        }
        self
    }

    pub fn pred(&self) -> Option<ListNodePosi<T>> {
        self.pr.upgrade()
    }

    pub fn suss(&self) -> Option<ListNodePosi<T>> {
        self.su.upgrade()
    }

}

impl<T> Deref for ListNode<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.data
    }
}

impl<T> DerefMut for ListNode<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.data
    }
}

impl<T: fmt::Debug> fmt::Debug for ListNode<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res = write!(f, "");
        let mut p = self.get_posi();
        let mut q = self.pred();
        let mut preds: Vec<String> = vec![];

        // collect preds
        while let Some(rc) = q {
            preds.push(format!("{:?}", **rc.borrow()));
            q = rc.borrow().pred();
        }

        // print preds
        for pred in preds.iter().rev() {
            res = write!(f, "{} -> ", pred);
        }
        
        // mark this
        res = write!(f, "(*)");

        // print suss
        while let Some(rc) = p {
            res = write!(f, "{:?}", **rc.borrow());
            p = rc.borrow().suss();
            if p.is_some() {
                res = write!(f, " -> ");
            }
        }
        res
    }
}